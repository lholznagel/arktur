use carina_config::CarinaConfig;
use carina_core_protocol::{decrypt, Events};
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub fn start(
    carina_config: Arc<Mutex<CarinaConfig>>,
    socket: UdpSocket,
) -> JoinHandle<()> {
    debug!("[THREAD_UDP] Starting udp thread");
    // the thread should run until the end
    thread::spawn(move || {
        let config = {
            let carina_config = match carina_config.lock() {
                Ok(s)  => s,
                Err(e) => panic!("[THREAD_UDP] Error locking carina_config: {}", e),
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

                    debug!(
                        "[THREAD_UDP] Received message from {}. Message: {:?}",
                        source, updated_buffer
                    );
                    let parsed = match config.peers.get(&(source.to_string())) {
                        Some(peer) => {
                            let parsed = decrypt(
                                &updated_buffer,
                                &config.nacl,
                                &peer.public_key,
                            );

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
                            let mut config = {
                                let carina_config = carina_config.lock().unwrap();
                                carina_config.config.clone()
                            };

                            let mut state = carina_config.lock().unwrap();
                            match state.events.get_mut(&Events::as_enum(buf[1])) {
                                Some(ref mut events) => {
                                    for i in 0..events.len() {
                                        match events[i].lock() {
                                            Ok(mut event) => {
                                                match event.execute(socket.try_clone().unwrap(), source.to_string(), &mut config, &buf[2..]) {
                                                    Err(e) => error!("[THREAD_UDP] Error calling execute {:?}", e),
                                                    _      => ()
                                                }
                                            },
                                            Err(_)        => error!("[THREAD_UDP] Error locking mutex.")
                                        };
                                    }
                                },
                                None         => ()
                            };
                        }
                        None => (),
                    }
                }
                Err(e) => error!("Error: {:?}", e),
            };
        }
    })
}
