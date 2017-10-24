#![deny(missing_docs)]

//! Hole puncher service
//!
//! Saved the last address that registered itself
extern crate blockchain_network;

use blockchain_network::udp_client::UdpClientBuilder;
use blockchain_network::event::EventHandler;

use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::net::{UdpSocket, SocketAddr};

/// Starting point
fn main() {
    remove_file("last_peer").unwrap();
    File::create("last_peer").unwrap();

    println!("Starting hole puncher!");

    let event_handlers = EventHandler::new()
        .set_register_handler(register_handler);
    
    UdpClientBuilder::new()
        .set_port(45000)
        .build(event_handlers)
        .listen();
}

/// Handler for the REGISTER event
fn register_handler(source: SocketAddr, udp: &UdpSocket, message: &str) {
    let mut file = File::open("last_peer").unwrap();
    let mut content = String::from("");
    let response;
    println!("Hole puncher: {:?}", message);
    
    file.read_to_string(&mut content).unwrap();

    if content == "" {
        response = "NO_PEER";
    } else {
        response = content.as_str();
        udp.send_to(("PEER_REGISTERING | ".to_owned() + source.to_string().as_str()).as_bytes(), content.parse::<SocketAddr>().unwrap()).unwrap();
    }

    let mut file = File::create("last_peer").unwrap();
    file.write_all(source.to_string().as_bytes()).unwrap();

    println!("Hole puncher response {:?}", "ACK_REGISTER | ".to_owned() + response);
    // for now we use static ip and port
    udp.send_to(("ACK_REGISTER | ".to_owned() + response).as_bytes(), source).unwrap();
}