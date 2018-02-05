use event_codes::EventCodes;
use hooks::Hooks;
use std::net::UdpSocket;

pub struct HookNotification {
    hook: Box<Hooks>
}

impl HookNotification {
    pub fn new(hook: Box<Hooks>) -> Self {
        Self {
            hook
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
    pub fn notify(&mut self, udp: &UdpSocket, event: EventCodes, payload_buffer: Vec<u8>, source: String) {
        match event {
            EventCodes::Ping => self.hook.on_ping(udp, payload_buffer, source),
            EventCodes::Pong => self.hook.on_pong(udp, payload_buffer, source),
            EventCodes::RegisterHolePuncher => self.hook.on_register_hole_puncher(udp, payload_buffer, source),
            EventCodes::RegisterHolePuncherAck => self.hook.on_register_hole_puncher_ack(udp, payload_buffer, source),
            EventCodes::RegisterPeer => self.hook.on_register_peer(udp, payload_buffer, source),
            EventCodes::RegisterPeerAck => self.hook.on_register_peer_ack(udp, payload_buffer, source),
            EventCodes::DataForBlock => self.hook.on_data_for_block(udp, payload_buffer, source),
            EventCodes::NewBlock => self.hook.on_new_block(udp, payload_buffer, source),
            EventCodes::PossibleBlock => self.hook.on_possible_block(udp, payload_buffer, source),
            EventCodes::ValidateHash => self.hook.on_validate_hash(udp, payload_buffer, source),
            EventCodes::ValidatedHash => self.hook.on_validated_hash(udp, payload_buffer, source),
            EventCodes::FoundBlock => self.hook.on_found_block(udp, payload_buffer, source),
            EventCodes::ExploreNetwork => self.hook.on_explore_network(udp, payload_buffer, source),
            EventCodes::NotAValidEvent => (),
        };
    }
}