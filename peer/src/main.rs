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

//! Terminal client application
//!
//! Connects with the connection manager and to other peers
//! Calculates the hash value for a new block
extern crate blockchain_hooks;
#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_protocol;
extern crate clap;
extern crate crypto;
extern crate futures_cpupool;
extern crate time;

use blockchain_hooks::{as_number, as_enum, EventCodes, Hooks, HookRegister};
use blockchain_protocol::Protocol;
use blockchain_protocol::payload::EmptyPayload;

use clap::{Arg, App};
use futures_cpupool::CpuPool;

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

/// Contains all hook implementations
mod hooks;
mod threads;

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
        .arg(Arg::with_name("STORAGE")
            .value_name("storage")
            .help("Sets the location for the blocks.")
            .takes_value(true)
            .long("storage")
            .default_value("./block_data"))
        .get_matches();

    let mut hole_puncher = String::from("");
    hole_puncher.push_str(matches.value_of("HOLE_PUNCHER_IP").unwrap());
    hole_puncher.push_str(":");
    hole_puncher.push_str(matches.value_of("HOLE_PUNCHER_PORT").unwrap());

    connect(hole_puncher, matches.value_of("STORAGE").unwrap().to_string());
}

/// Builds up a UDP connection with the hole_puncher
fn connect(hole_puncher: String, storage: String) {
    info!("Hole puncher: {:?}", hole_puncher.clone());
    let pool = CpuPool::new_num_cpus();
    let mut thread_storage = Vec::new();

    let hooks = Hooks::new()
        .set_block_data(hooks::blocks::block_data)
        .set_block_found(hooks::blocks::block_found)
        .set_block_gen(hooks::blocks::block_gen)
        .set_get_block(hooks::blocks::get_block)
        .set_get_block_ack(hooks::blocks::get_block_ack)
        .set_get_blocks(hooks::blocks::get_blocks)
        .set_get_blocks_ack(hooks::blocks::get_blocks_ack)
        .set_get_peers(hooks::peers::get_peers)
        .set_get_peers_ack(hooks::peers::get_peers_ack)
        .set_hash_val(hooks::blocks::hash_val)
        .set_hash_val_ack(hooks::blocks::hash_val_ack)
        .set_punsh(hooks::misc::punsh)
        .set_ping(hooks::misc::ping)
        .set_pong(hooks::misc::pong)
        .set_register(hooks::peers::register)
        .set_register_ack(hooks::peers::register_ack);

    let state = Arc::new(Mutex::new(hooks::State::new(storage)));

    let request = Protocol::<EmptyPayload>::new()
        .set_event_code(as_number(EventCodes::Register))
        .build();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Binding an UdpSocket should be successful.");
    socket.send_to(request.as_slice(), hole_puncher.clone()).expect("Sending a request should be successful.");

    let udp_clone_peer = socket.try_clone().expect("Cloning the UPD connection failed.");
    thread_storage.push(threads::peer_sync(&pool, Arc::clone(&state), udp_clone_peer));

    let udp_clone_peer_ping = socket.try_clone().expect("Cloning the UPD connection failed.");
    thread_storage.push(threads::peer_ping(&pool, Arc::clone(&state), udp_clone_peer_ping));

    let udp_clone_block = socket.try_clone().expect("Cloning the UPD connection failed.");
    thread_storage.push(threads::block(&pool, Arc::clone(&state), udp_clone_block));

    let mut hook_notification = HookRegister::new(hooks, state)
        .get_notification();

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