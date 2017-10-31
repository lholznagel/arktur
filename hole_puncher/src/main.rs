#![deny(missing_docs)]

//! Hole puncher service
//!
//! Saved the last address that registered itself
#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_network;
extern crate blockchain_protocol;

use blockchain_network::udp_client::UdpClientBuilder;
use blockchain_network::event::EventHandler;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::events::EventCodes;

use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::net::{UdpSocket, SocketAddr};

/// Starting point
fn main() {
    remove_file("last_peer").unwrap();
    File::create("last_peer").unwrap();

    info!("Starting hole puncher!");

    let event_handlers = EventHandler::new().set_register_handler(register_handler);

    UdpClientBuilder::new()
        .set_port(45000)
        .build(event_handlers)
        .listen();
}

/// Handler for the REGISTER event
fn register_handler(source: SocketAddr, udp: &UdpSocket, _: BlockchainProtocol) {
    let mut file = File::open("last_peer").unwrap();
    let mut content = String::from("");
    let response;

    file.read_to_string(&mut content).unwrap();

    if content == "" {
        response = "NO_PEER";
    } else {
        response = content.as_str();

        let message = BlockchainProtocol::new()
            .set_event_code(EventCodes::PeerRegistering)
            .set_data(source.to_string())
            .build();
        udp.send_to(message.as_slice(), content.parse::<SocketAddr>().unwrap())
            .unwrap();
    }

    let mut file = File::create("last_peer").unwrap();
    file.write_all(source.to_string().as_bytes()).unwrap();

    sending!(format!("ACK_REGISTER | {}", response));
    
    let message = BlockchainProtocol::new().set_event_code(EventCodes::AckRegister).set_data(String::from(response)).build();
    udp.send_to(message.as_slice(), source)
        .unwrap();
}