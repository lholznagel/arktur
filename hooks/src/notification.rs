use hook_codes::HookCodes;
use hooks::Hooks;
use message_state::MessageState;

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
    pub fn notify(&mut self, udp: UdpSocket, event: HookCodes, payload_buffer: Vec<u8>, source: String) {
        let udp_clone = udp.try_clone().expect("Cloning the current UDP connection should be successful");

        let state = MessageState {
            payload_buffer,
            source,
            state: Arc::clone(&self.state),
            udp: udp_clone,
        };

        let event_match = match event {
            HookCodes::Ping => self.hook.ping,
            HookCodes::Pong => self.hook.pong,
            HookCodes::Register => self.hook.register,
            HookCodes::RegisterAck => self.hook.register_ack,
            HookCodes::GetBlocks => self.hook.get_blocks,
            HookCodes::GetBlocksAck => self.hook.get_blocks_ack,
            HookCodes::GetBlock => self.hook.get_block,
            HookCodes::GetBlockAck => self.hook.get_block_ack,
            HookCodes::BlockData => self.hook.block_data,
            HookCodes::BlockGen => self.hook.block_gen,
            HookCodes::BlockFound => self.hook.block_found,
            HookCodes::HashVal => self.hook.hash_val,
            HookCodes::HashValAck => self.hook.hash_val_ack,
            HookCodes::NotAValidType => None,
        };

        let thread = self.pool.spawn_fn(move || {
            event_match.map(|hook| (hook)(state));

            let res: Result<bool, ()> = Ok(true);
            res
        });

        self.threads.push(thread);
    }
}