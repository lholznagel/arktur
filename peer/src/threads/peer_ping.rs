use blockchain_hooks::{as_number, EventCodes};
use blockchain_protocol::Protocol;
use blockchain_protocol::payload::{EmptyPayload, Payload};

use hooks::State;
use futures_cpupool::{CpuFuture, CpuPool};

use std::{thread, time};
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

pub fn peer_ping(cpu_pool: &CpuPool, state: Arc<Mutex<State>>, udp: UdpSocket) -> CpuFuture<bool, ()> {
    #[allow(unreachable_code)]
    cpu_pool.spawn_fn(move || {
        loop {
            // ping every 60 seconds
            thread::sleep(time::Duration::from_secs(60));

            {
                let mut state_lock = state.lock().unwrap();

                for (peer, counter) in state_lock.peers.clone() {
                    // if we pinged him 6 times he is considered dead
                    if counter == 6 {
                        state_lock.peers.remove(&peer);
                        info!("Peer did not answer. He´s dead Jimmy :(");
                    } else {
                        state_lock.peers.insert(peer.clone(), counter + 1);

                        let message = Protocol::new()
                            .set_event_code(as_number(EventCodes::Ping))
                            .set_payload(EmptyPayload::new())
                            .build();

                        udp.send_to(&message, peer).expect("Sending a UDP message should be successful");
                    }
                }
            }
        }

        let res: Result<bool, ()> = Ok(true);
        res
    })
}