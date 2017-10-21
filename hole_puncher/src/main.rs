extern crate blockchain_network;

use blockchain_network::udp_client::UdpClientBuilder;
use blockchain_network::event::EventHandler;

use std::net::{UdpSocket, SocketAddr};

/// Starting point
fn main() {
    println!("Starting hole puncher!");

    let event_handlers = EventHandler::new()
        .set_register_handler(register_handler);
    
    UdpClientBuilder::new()
        .set_port(45000)
        .build(event_handlers)
        .listen();
}

/// Handler for the REGISTER event
fn register_handler(source: SocketAddr, udp: &UdpSocket, _: &str) {
    // for now we use static ip and port
    udp.send_to("127.0.0.1:45001".as_bytes(), source).unwrap();
}