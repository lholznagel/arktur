use event_codes::EventCodes;
use hooks::Hooks;
use state::ApplicationState;

use futures_cpupool::{CpuFuture, CpuPool};

use std::sync::{Arc, Mutex};
use std::net::UdpSocket;

pub struct HookNotification<T> {
    hook: Hooks<T>,
    state: Arc<Mutex<T>>,
    pool: CpuPool,
    threads: Vec<CpuFuture<bool, ()>>
}

impl<T: 'static> HookNotification<T> where T: Send {
    pub fn new(hook: Hooks<T>, state: Arc<Mutex<T>>) -> Self {
        Self {
            hook,
            state,
            pool: CpuPool::new_num_cpus(),
            threads: Vec::new()
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

        let event_match = match event {
            EventCodes::Ping => self.hook.on_ping,
            EventCodes::Pong => self.hook.on_pong,
            EventCodes::RegisterHolePuncher => self.hook.on_register_hole_puncher,
            EventCodes::RegisterHolePuncherAck => self.hook.on_register_hole_puncher_ack,
            EventCodes::RegisterPeer => self.hook.on_register_peer,
            EventCodes::RegisterPeerAck => self.hook.on_register_peer_ack,
            EventCodes::DataForBlock => self.hook.on_data_for_block,
            EventCodes::NewBlock => self.hook.on_new_block,
            EventCodes::PossibleBlock => self.hook.on_possible_block,
            EventCodes::ValidateHash => self.hook.on_validate_hash,
            EventCodes::ValidatedHash => self.hook.on_validated_hash,
            EventCodes::FoundBlock => self.hook.on_found_block,
            EventCodes::SyncPeers => self.hook.on_sync_peers,
            EventCodes::ExploreNetwork => self.hook.on_explore_network,
            EventCodes::NotAValidEvent => None,
        };

        let thread = self.pool.spawn_fn(move || {
            match event_match {
                Some(hook) => {
                    (hook)(state);
                },
                None => ()
            };

            let res: Result<bool, ()> = Ok(true);
            res
        });

        self.threads.push(thread);
    }
}