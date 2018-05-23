use futures_cpupool::{CpuFuture, CpuPool};
use state::State;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

pub fn start(cpu_pool: &CpuPool, state: Arc<Mutex<State>>) -> CpuFuture<bool, ()> {
    debug!("[THREAD UDP] Starting udp thread");
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
        let socket = UdpSocket::bind(config.uri)
                .expect("Binding an UdpSocket should be successful.");

        debug!("[THREAD UDP] Starting udp listener");
        loop {
            let mut buffer = [0; 65535];

            match socket.recv_from(&mut buffer) {
                Ok((bytes, source)) => {
                    info!("Received message from {}. Message: {}", source, bytes)
                }
                Err(e) => error!("Error: {:?}", e),
            }
        }

        let res: Result<bool, ()> = Ok(true);
        res
    })
}
