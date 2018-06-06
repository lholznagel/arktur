use carina_core_protocol;
use event::as_enum;
use futures_cpupool::{CpuFuture, CpuPool};
use carina_config::CarinaConfig;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

pub fn start(
    carina_config: Arc<Mutex<CarinaConfig>>,
    cpu_pool: &CpuPool,
    socket: UdpSocket,
) -> CpuFuture<bool, ()> {
    debug!("[THREAD_UDP] Starting udp thread");
    // the thread should run until the end
    #[allow(unreachable_code)]
    cpu_pool.spawn_fn(move || {
        let config = {
            let carina_config = match carina_config.lock() {
                Ok(s) => s,
                Err(e) => panic!("Error locking carina_config: {}", e),
            };
            carina_config.config.clone()
        };

        debug!("[THREAD_UDP] Starting udp listener");
        loop {
            let mut buffer = [0; 65535];

            match socket.recv_from(&mut buffer) {
                Ok((bytes, source)) => {
                    let mut updated_buffer = Vec::new();
                    for i in 0..bytes {
                        updated_buffer.push(buffer[i])
                    }

                    info!(
                        "[THREAD_UDP] Received message from {}. Message: {:?}",
                        source, updated_buffer
                    );
                    let parsed = match config.peers.get(&(source.to_string())) {
                        Some(peer) => {
                            let parsed = carina_core_protocol::decrypt(
                                &updated_buffer,
                                &config.nacl,
                                &peer.public_key,
                            );
                            info!("[THREAD_UDP] {:?}", parsed);
                            
                            if parsed.is_ok() {
                                Some(parsed.unwrap())
                            } else {
                                None
                            }
                        }
                        None => {
                            info!("[THREAD_UDP] Didn´t find peer");
                            None
                        }
                    };

                    match parsed {
                        Some(buf) => {
                            let events = {
                                let carina_config = carina_config.lock().unwrap();
                                carina_config.events.clone()
                            };

                            // TODO: implement event

                            match events.get(&as_enum(buf[1])) {
                                Some(events) => {
                                    for event in events {
                                        event.execute();
                                    }
                                },
                                None    => ()
                            };
                        }
                        None => (),
                    }
                }
                Err(e) => error!("Error: {:?}", e),
            };
        }

        let res: Result<bool, ()> = Ok(true);
        res
    })
}
