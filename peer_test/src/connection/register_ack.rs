use carina_hooks::{as_number, EventCodes};
use carina_peer;
use carina_peer::config::{Config, HolePuncher};
use carina_protocol::payload::EmptyPayload;
use carina_protocol::payload::peers::GetPeersAck;
use carina_protocol::Protocol;

use futures_cpupool::{CpuFuture, CpuPool};
use test_case::TestCase;

use std::{thread, time};
use std::net::UdpSocket;

pub struct RegisterAck;

impl TestCase for RegisterAck {
    fn description() -> String {
        String::from("Tests if register works.")
    }

    fn name() -> String {
        String::from("Register ack")
    }

    fn execute(cpu_pool: &CpuPool) -> CpuFuture<bool, ()> {
        cpu_pool.spawn_fn(move || {
            println!("Starting peer");

            thread::spawn(|| {
                let config = Config {
                    hole_puncher: HolePuncher::new(),
                    port: 12345,
                    storage: String::from("not_needed")
                };
                carina_peer::init(config);
            });
            // wait 1 second to let the peer startup
            thread::sleep(time::Duration::from_secs(1));

            println!("Peer started");

            let socket = UdpSocket::bind("0.0.0.0:0").expect("Binding an UdpSocket should be successful.");

            let request = Protocol::<EmptyPayload>::new()
                .set_event_code(as_number(EventCodes::Register))
                .build();
            socket.send_to(request.as_slice(), "0.0.0.0:12345").expect("Sending a request should be successful.");

            let mut buffer = [0; 65535];

            match socket.recv_from(&mut buffer) {
                Ok((bytes, _)) => {
                    let mut updated_buffer = Vec::new();
                    for i in 0..bytes {
                        updated_buffer.push(buffer[i])
                    }

                    let message = Protocol::<GetPeersAck>::from_bytes(&updated_buffer)
                        .expect("Parsing the protocol should be successful.");

                    assert_eq!(message.payload.peers.len(), 0);
                    println!("Test ok");
                    Ok(true)

                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    Ok(false)
                },
            }
        })
    }
}