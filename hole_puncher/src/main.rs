#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! Hole puncher service
//!
//! Initiates a connection between two peers behind a NAT
//! Also saves all registered peers and provides them to new peers
//!
//! # Hole puncher
//!
//! - Create a "hole" between to peers
//! - When a peer registers itself, its IP-Address + Port are saved
//! - The next peer that registers itself, gets these IP-Address + Port
//! - The older peer gets the IP-Address + Port of the new peer
//! - The address of the new peer are saved instead of the old peer
//! - Both start a ping event to the other peer
//! - With this technic a connection between two private networks can be accomplished
//!
//! In the following graphic, the process is shown
//!
//! ```
//!  1. Register  +--------------+ 2. Register
//!   +--------->|              |<---------+
//!   |          | hole puncher |          |
//!   |    +-----|              |-----+    |
//!   |    |     +--------------+     |    |
//!   |    | 3. Send IP+Port of new   |    |
//!   |    |                          |    |
//!   |    |                          |    |
//!   |    |                          |    |
//!   |    |   4. Send IP+Port of old |    |
//!   |    v                          v    |
//! +--------+                      +--------+
//! |        |--------------------->|        |
//! | Peer A |      5. Contact      | Peer B |
//! |        |<---------------------|        |
//! +--------+                      +--------+
//!
//! created with http://asciiflow.com/
//! ```
//!
//! # Example
//!
//! - Peer A runs on 192.168.1.5:45678 (on host a)
//! - Peer B runs on 192.168.1.6:56789 (on host b)
//! - Peer A registers itself at the hole puncher (some.public.ip.address:45000)
//! - The hole puncher does not know any peer
//! - Peer B registers itself at the same hole puncher
//! - The hole puncher sends the Peer B information to Peer A
//! - The hole puncher then sends the Peer A information to Peer B
//! - Peer A and Peer B try to ping each other
//! - The connection between both networks should be good to go
//!
//! Default port: 50000
extern crate carina_hooks;
#[macro_use]
extern crate log;
extern crate carina_protocol;
extern crate futures_cpupool;
extern crate sodiumoxide;

mod hooks;

use carina_hooks::{as_number, as_enum, EventCodes, Hooks, HookRegister};
use carina_protocol::Protocol;
use carina_protocol::payload::{EmptyPayload, Payload};

use futures_cpupool::CpuPool;

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    info!("Starting hole puncher on port 50000");
    connect();
}

fn connect() {
    let pool = CpuPool::new_num_cpus();
    let mut threads = Vec::new();

    let hooks = Hooks::new()
        .set_pong(hooks::on_pong)
        .set_punsh(hooks::punsh)
        .set_register(hooks::register)
        .set_get_peers(hooks::get_peers);

    let state = Arc::new(Mutex::new(hooks::State::new()));
    let state_clone_peer_ping = Arc::clone(&state);
    let mut hook_notification = HookRegister::new(hooks, Arc::clone(&state))
        .get_notification();

    let socket = UdpSocket::bind("0.0.0.0:50000").expect("Binding an UdpSocket should be successful.");

    let udp_clone_peer_ping = socket.try_clone().expect("Cloning the UPD connection failed.");
    #[allow(unreachable_code)]
    let peer_ping = pool.spawn_fn(move || {
        loop {
            // ping every 30 seconds
            thread::sleep(time::Duration::from_secs(30));

            {
                let mut nacl = {
                    let state_lock = state_clone_peer_ping.lock()
                        .expect("Locking the mutex should be successful.");
                    state_lock.nacl.clone()
                };

                let mut state_lock = state_clone_peer_ping.lock().unwrap();

                for (peer, (public_key, counter)) in state_lock.peers.clone() {
                    // if we pinged him 6 times he is considered dead
                    if counter == 6 {
                        state_lock.peers.remove(&peer);
                        info!("Peer did not answer. Removing. He´s dead Jimmy :(");
                    } else {
                        state_lock.peers.insert(peer.clone(), (public_key, counter + 1));

                        let message = Protocol::new()
                            .set_event_code(as_number(EventCodes::Ping))
                            .set_payload(EmptyPayload::new())
                            .build(&mut nacl, &public_key);

                        udp_clone_peer_ping.send_to(&message, peer).expect("Sending a UDP message should be successful");
                    }
                }
            }
        }

        let res: Result<bool, ()> = Ok(true);
        res
    });
    threads.push(peer_ping);

    loop {
        let mut buffer = [0; 65535];

        match socket.recv_from(&mut buffer) {
            Ok((bytes, source)) => {
                let mut updated_buffer = Vec::new();
                for i in 0..bytes {
                    updated_buffer.push(buffer[i])
                }

                let mut nacl = {
                    let state_lock = state.lock()
                        .expect("Locking the mutex should be successful.");
                    state_lock.nacl.clone()
                };
                let updated_buffer = {
                    let state_lock = state.lock()
                        .expect("Locking the mutex should be successful.");

                    match state_lock.peers.get(&source.to_string()) {
                        Some(peer) => carina_protocol::parse_encrypted(&updated_buffer, &nacl, &peer.0),
                        None => updated_buffer[24..].to_vec()
                    }
                };

                let socket_clone = socket.try_clone().expect("Cloning the socket should be successful.");
                hook_notification.notify(socket_clone, as_enum(updated_buffer[1]), updated_buffer, source.to_string());
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}