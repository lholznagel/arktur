use futures_cpupool::{CpuFuture, CpuPool};
use state::State;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

pub fn start(cpu_pool: &CpuPool, state: Arc<Mutex<State>>) -> CpuFuture<bool, ()> {
    debug!("[THREAD_UDP] Starting udp thread");
    // the thread should run until the end
    #[allow(unreachable_code)]
    cpu_pool.spawn_fn(move || {
        let config = {
            let state = match state.lock() {
                Ok(s) => s,
                Err(e) => panic!("Error locking state: {}", e)
            };
            state.config.clone()
        };
        let socket = UdpSocket::bind(&config.uri).unwrap();
        info!("[THREAD_UDP] Listening on  {}", config.uri);

        debug!("[THREAD_UDP] Starting udp listener");
        loop {
            let mut buffer = [0; 65535];

            match socket.recv_from(&mut buffer) {
                Ok((bytes, source)) => {
                    info!("[THREAD_UDP] Received message from {}. Message: {}", source, bytes);
                    match config.peers.get(&(source.to_string())) {
                        Some(peer) => info!("[THREAD_UDP] {:?}", peer),
                        None => info!("[THREAD_UDP] Didn´t find peer")
                    }
                }
                Err(e) => error!("Error: {:?}", e),
            };
        }

        let res: Result<bool, ()> = Ok(true);
        res
    })
}
