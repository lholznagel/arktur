use event_codes::EventCodes;
use hooks::Hooks;
use state::ApplicationState;

use futures_cpupool::{CpuFuture, CpuPool};

use std::sync::{Arc, Mutex};
use std::net::UdpSocket;

/// Contains all registered hooks
/// Notifies the hooks on incoming messages and gives them a global state
#[derive(Debug)]
pub struct HookNotification<T> {
    hook: Hooks<T>,
    state: Arc<Mutex<T>>,
    pool: CpuPool,
    threads: Vec<CpuFuture<bool, ()>>
}

impl<T: 'static> HookNotification<T> where T: Send {
    /// Creates a new HookNotification instance
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
            EventCodes::Ping => self.hook.ping,
            EventCodes::Pong => self.hook.pong,
            EventCodes::Punsh => self.hook.punsh,
            EventCodes::GetPeers => self.hook.get_peers,
            EventCodes::GetPeersAck => self.hook.get_peers_ack,
            EventCodes::Register => self.hook.register,
            EventCodes::RegisterAck => self.hook.register_ack,
            EventCodes::GetBlocks => self.hook.get_blocks,
            EventCodes::GetBlocksAck => self.hook.get_blocks_ack,
            EventCodes::GetBlock => self.hook.get_block,
            EventCodes::GetBlockAck => self.hook.get_block_ack,
            EventCodes::BlockData => self.hook.block_data,
            EventCodes::BlockGen => self.hook.block_gen,
            EventCodes::BlockFound => self.hook.block_found,
            EventCodes::HashVal => self.hook.hash_val,
            EventCodes::HashValAck => self.hook.hash_val_ack,
            EventCodes::NotAValidType => None,
        };

        let thread = self.pool.spawn_fn(move || {
            event_match.map(|hook| (hook)(state));

            let res: Result<bool, ()> = Ok(true);
            res
        });

        self.threads.push(thread);
    }
}