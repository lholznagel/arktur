#![deny(missing_docs)]

//! Terminal client application
//!
//! Connects with the connection manager and to other peers
//! Calculates the hash value for a new block
extern crate blockchain_file;
extern crate blockchain_hooks;
#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_protocol;
extern crate clap;
extern crate futures_cpupool;
extern crate crypto;

use blockchain_hooks::{as_enum, EventCodes, Hooks, HookRegister};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::{RegisterPayload, SyncPeersPayload, Payload};
use blockchain_protocol::enums::status::StatusCodes;

use clap::{Arg, App};
use futures_cpupool::CpuPool;

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::{thread, time};

/// Contains all handlers the peer listens to
mod handlers;

fn main() {
    let matches = App::new("Blockchain network cli")
        .version("0.1.0")
        .author("Lars Holznagel")
        .about("Client tool for rust-blockchain")
        .arg(Arg::with_name("HOLE_PUNCHER_IP")
            .value_name("ip")
            .help("Sets the IP of the Hole puncher service")
            .takes_value(true)
            .required(true)
            .long("puncher_ip")
            .default_value("0.0.0.0"))
        .arg(Arg::with_name("HOLE_PUNCHER_PORT")
            .value_name("port")
            .help("Sets the port of the Hole puncher service.")
            .takes_value(true)
            .long("puncher_port")
            .default_value("50000"))
        .get_matches();

    let mut hole_puncher = String::from("");
    hole_puncher.push_str(matches.value_of("HOLE_PUNCHER_IP").unwrap());
    hole_puncher.push_str(":");
    hole_puncher.push_str(matches.value_of("HOLE_PUNCHER_PORT").unwrap());
    connect(hole_puncher);
}

/// Builds up a UDP connection with the connection manager
fn connect(hole_puncher: String) {
    let pool = CpuPool::new_num_cpus();
    let mut threads = Vec::new();

    let hooks = Hooks::new()
        .set_ping(handlers::on_ping)
        .set_pong(handlers::on_pong)
        .set_register_hole_puncher_ack(handlers::on_register_hole_puncher_ack)
        .set_register_peer(handlers::on_register_peer)
        .set_register_peer_ack(handlers::on_register_peer_ack)
        .set_data_for_block(handlers::on_data_for_block)
        .set_new_block(handlers::on_new_block)
        .set_validate_hash(handlers::on_validate_hash)
        .set_found_block(handlers::on_found_block)
        .set_sync_peers(handlers::on_sync_peers)
        .set_explore_network(handlers::on_explore_network);

    let state_handler = handlers::StateHandler::new();
    let state = Arc::new(Mutex::new(state_handler));
    let state_clone = Arc::clone(&state);

    info!("Hole puncher: {:?}", hole_puncher);
    let mut hook_notification = HookRegister::new(hooks, state)
        .get_notification();

    let request = BlockchainProtocol::<RegisterPayload>::new()
        .set_event_code(EventCodes::RegisterHolePuncher)
        .set_status_code(StatusCodes::Ok)
        .build();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Binding an UdpSocket should be successful.");
    socket.send_to(request.as_slice(), hole_puncher).expect("Sending a request should be successful");

    let udp_clone = socket.try_clone().expect("Cloning the UPD connection failed.");
    #[allow(unreachable_code)]
    let thread = pool.spawn_fn((move || {
        loop {
            // sync every 2 minutes
            thread::sleep(time::Duration::from_secs(120));

            {
                let state_lock = state_clone.lock().unwrap();
                for peer in state_lock.peers.clone() {
                    let message = BlockchainProtocol::new()
                        .set_event_code(EventCodes::SyncPeers)
                        .set_status_code(StatusCodes::Ok)
                        .set_payload(SyncPeersPayload::new().set_peers(state_lock.peers.clone()))
                        .build();

                    udp_clone.send_to(&message, peer).expect("Sending a UDP message should be successful");
                }
            }
        }

        let res: Result<bool, ()> = Ok(true);
        res
    }));

    threads.push(thread);

    loop {
        let mut buffer = [0; 65535];

        match socket.recv_from(&mut buffer) {
            Ok((bytes, source)) => {
                let mut updated_buffer = Vec::new();
                for i in 0..bytes {
                    updated_buffer.push(buffer[i])
                }

                let socket_clone = socket.try_clone().expect("Cloning the socket should be successful.");
                hook_notification.notify(socket_clone, as_enum(updated_buffer[0]), updated_buffer, source.to_string());
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}