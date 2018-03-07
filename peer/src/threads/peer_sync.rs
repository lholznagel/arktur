use blockchain_hooks::{as_number, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{Payload, EmptyPayload};

use hooks::State;
use futures_cpupool::{CpuFuture, CpuPool};

use std::{thread, time};
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

pub fn peer_sync(cpu_pool: &CpuPool, state: Arc<Mutex<State>>, udp: UdpSocket) -> CpuFuture<bool, ()> {
    #[allow(unreachable_code)]
    cpu_pool.spawn_fn(move || {
        loop {
            // sync every 5 minutes
            thread::sleep(time::Duration::from_secs(10));

            {
                let state_lock = state.lock().unwrap();

                for (peer, _) in state_lock.peers.clone() {
                    let message = BlockchainProtocol::new()
                        .set_event_code(as_number(EventCodes::GetPeers))
                        .set_status_code(StatusCodes::Ok)
                        .set_payload(EmptyPayload::new())
                        .build();

                    udp.send_to(&message, peer).expect("Sending a UDP message should be successful");
                }
            }
        }

        let res: Result<bool, ()> = Ok(true);
        res
    })
}