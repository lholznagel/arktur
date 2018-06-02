use carina_core_protocol;
use carina_core_protocol::{MessageBuilder, Payload};
use futures_cpupool::{CpuFuture, CpuPool};
use state::State;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

/// Start a new thread that executes a ping event to all
/// peers in the config every 60 seconds
pub fn start(
    cpu_pool: &CpuPool,
    state: Arc<Mutex<State>>,
    socket: UdpSocket,
) -> CpuFuture<bool, ()> {
    debug!("[THREAD_PING] Starting ping thread");
    // the thread should run until the program is killed
    #[allow(unreachable_code)]
    cpu_pool.spawn_fn(move || {
        let mut config = {
            let state = match state.lock() {
                Ok(s) => s,
                Err(e) => panic!("Error locking state: {}", e),
            };
            state.config.clone()
        };

        loop {
            for (_, peer) in &config.peers {
                let message = MessageBuilder::new()
                    .set_event_code(0)
                    .set_payload(carina_core_protocol::payloads::EmptyPayload::new())
                    .build(&mut config.nacl, &peer.public_key);

                match socket.send_to(&message, &peer.address) {
                    Ok(_)  => debug!("[THREAD_PING] Send ping to peer {}", peer.address),
                    Err(e) => error!("[THREAD_PING] Error sending ping to peer: {}. Error: {}", peer.address, e),
                };
            }

            debug!("[THREAD_PING] Waiting 60 seconds until next ping.");
            thread::sleep(time::Duration::from_secs(60));
        }

        let res: Result<bool, ()> = Ok(true);
        res
    })
}
