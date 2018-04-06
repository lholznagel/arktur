use carina_hooks::{as_number, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::{Payload, EmptyPayload};

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
                    let message = Protocol::new()
                        .set_event_code(as_number(EventCodes::GetPeers))
                        .set_payload(EmptyPayload::new())
                        .build(&state_lock.nacl);

                    udp.send_to(&message, peer).expect("Sending a UDP message should be successful");
                }
            }
        }

        let res: Result<bool, ()> = Ok(true);
        res
    })
}