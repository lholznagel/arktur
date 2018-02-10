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
extern crate crypto;

use blockchain_hooks::{as_enum, EventCodes, HookRegister};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::RegisterPayload;
use blockchain_protocol::enums::status::StatusCodes;

use clap::{Arg, App};

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

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
    info!("Hole puncher: {:?}", hole_puncher);
    let state_handler = handlers::StateHandler::new();
    let mut hook_notification = HookRegister::new(Box::new(handlers::HookHandler), Arc::new(Mutex::new(state_handler)))
        .get_notification();

    let request = BlockchainProtocol::<RegisterPayload>::new()
        .set_event_code(EventCodes::RegisterHolePuncher)
        .set_status_code(StatusCodes::Ok)
        .build();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Binding an UdpSocket should be successful.");
    socket.send_to(request.as_slice(), hole_puncher).expect("Sending a request should be successful");

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