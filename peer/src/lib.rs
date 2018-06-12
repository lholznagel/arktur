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

//! Library that represents a blockchain peer

extern crate base64;
extern crate carina_hooks;
extern crate carina_protocol;
extern crate crypto;
extern crate futures_cpupool;
extern crate time;
#[macro_use]
extern crate log;
extern crate sodiumoxide;

use carina_hooks::{as_number, as_enum, HookCodes, Hooks, HookRegister};
use carina_protocol::Protocol;
use carina_protocol::payload::peers::Register;

use futures_cpupool::CpuPool;

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

/// Contains all hook implementations
pub mod config;
mod hooks;
mod threads;

/// Builds up a UDP connection with the hole_puncher
pub fn init(config: config::Config) {
    sodiumoxide::init().unwrap();

    let hooks = Hooks::new()
        .add(HookCodes::BlockData, hooks::blocks::block_data)
        .add(HookCodes::BlockFound, hooks::blocks::block_found)
        .add(HookCodes::BlockGen, hooks::blocks::block_gen)
        .add(HookCodes::GetBlock, hooks::blocks::get_block)
        .add(HookCodes::GetBlockAck, hooks::blocks::get_block_ack)
        .add(HookCodes::GetBlocks, hooks::blocks::get_blocks)
        .add(HookCodes::GetBlocksAck, hooks::blocks::get_blocks_ack)
        .add(HookCodes::HashVal, hooks::blocks::hash_val)
        .add(HookCodes::HashValAck, hooks::blocks::hash_val_ack)
        .add(HookCodes::Ping, hooks::misc::ping)
        .add(HookCodes::Pong, hooks::misc::pong)
        .add(HookCodes::Register, hooks::peers::register)
        .add(HookCodes::RegisterAck, hooks::peers::register_ack);

    connect(config, hooks);
}

/// Builds up a UDP connection with the hole_puncher
fn connect(config: config::Config, hooks: Hooks<hooks::State>) {
    let pool = CpuPool::new(4);
    let mut thread_storage = Vec::new();

    let state = Arc::new(Mutex::new(hooks::State::new(config.clone())));

    let register = Register {
        public_key: {
            let state_lock = state.lock().expect("Locking the mutex should be successful.");
            state_lock.nacl.get_public_key()
        }
    };
    let mut nacl = {
        let state_lock = state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };

    let request = Protocol::<Register>::new()
        .set_event_code(as_number(HookCodes::Register))
        .set_payload(register)
        .build_unencrypted(&mut nacl);

    let socket = UdpSocket::bind(format!("0.0.0.0:{}", config.port)).expect("Binding an UdpSocket should be successful.");
    for peer in config.peers {
        socket.send_to(request.as_slice(), peer.address).expect("Sending a request should be successful.");
    }

    let udp_clone_peer_ping = socket.try_clone().expect("Cloning the UPD connection failed.");
    thread_storage.push(threads::peer_ping(&pool, Arc::clone(&state), udp_clone_peer_ping));

    let udp_clone_block = socket.try_clone().expect("Cloning the UPD connection failed.");
    thread_storage.push(threads::block(&pool, Arc::clone(&state), udp_clone_block));

    let mut hook_notification = HookRegister::new(hooks, Arc::clone(&state)).get_notification();
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