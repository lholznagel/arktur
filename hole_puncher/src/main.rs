extern crate blockchain_network;

use blockchain_network::udp_client::UdpClientBuilder;
use blockchain_network::event::EventHandler;

use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};

/// Starting point
fn main() {
    remove_file("last_peer");
    File::create("last_peer");
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
    let mut response = "";
    println!("Hole puncher: {:?}", message);
    
    file.read_to_string(&mut content).unwrap();

    if content == "" {
        response = "NO_PEER";
    } else {
        response = content.as_str();
    }

    let mut file = File::create("last_peer").unwrap();
    file.write_all(source.to_string().as_bytes());

    println!("Hole puncher response {:?}", "ACK_REGISTER | ".to_owned() + response);
    // for now we use static ip and port
    udp.send_to(("ACK_REGISTER | ".to_owned() + response).as_bytes(), source).unwrap();
}