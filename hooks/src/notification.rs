use event_codes::EventCodes;
use hooks::Hooks;
use state::ApplicationState;

use std::sync::{Arc, Mutex};

use std::net::UdpSocket;

pub struct HookNotification<T> {
    hook: Box<Hooks<T>>,
    state: Arc<Mutex<T>>
}

impl<T> HookNotification<T> {
    pub fn new(hook: Box<Hooks<T>>, state: Arc<Mutex<T>>) -> Self {
        Self {
            hook,
            state
        }
    }

    /// Checks the event and executes the matching hook
    ///
    /// # Parameters
    ///
    /// - `udp` - Open UDP connection
    /// - `event` - Event code to check what event this is
    /// - `payload_buffer` - raw message
    /// - `source` - source this message comes from
    pub fn notify(&mut self, udp: UdpSocket, event: EventCodes, payload_buffer: Vec<u8>, source: String) {
        let udp_clone = udp.try_clone().expect("Cloning the current UDP connection should be successful");

        let state = ApplicationState {
            payload_buffer,
            source,
            state: Arc::clone(&self.state),
            udp: udp_clone,
        };

        match event {
            EventCodes::Ping => self.hook.on_ping(state),
            EventCodes::Pong => self.hook.on_pong(state),
            EventCodes::RegisterHolePuncher => self.hook.on_register_hole_puncher(state),
            EventCodes::RegisterHolePuncherAck => self.hook.on_register_hole_puncher_ack(state),
            EventCodes::RegisterPeer => self.hook.on_register_peer(state),
            EventCodes::RegisterPeerAck => self.hook.on_register_peer_ack(state),
            EventCodes::DataForBlock => self.hook.on_data_for_block(state),
            EventCodes::NewBlock => self.hook.on_new_block(state),
            EventCodes::PossibleBlock => self.hook.on_possible_block(state),
            EventCodes::ValidateHash => self.hook.on_validate_hash(state),
            EventCodes::ValidatedHash => self.hook.on_validated_hash(state),
            EventCodes::FoundBlock => self.hook.on_found_block(state),
            EventCodes::ExploreNetwork => self.hook.on_explore_network(state),
            EventCodes::NotAValidEvent => (),
        };
    }
}