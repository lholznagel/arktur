extern crate blockchain_network;
extern crate clap;

use blockchain_network::udp_client::UdpClientBuilder;
use blockchain_network::event::EventHandler;

use clap::{Arg, App};

use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

fn main() {
    let event_handler = EventHandler::new();
    let event_handler = event_handler
        .set_ping_handler(ping_handler)
        .set_pong_handler(pong_handler)
        .set_register_ack_handler(register_ack_handler);

    //UdpClientBuilder::new(event_handler);
    let udp_client = UdpClientBuilder::new().build(event_handler);
    let udp_client = udp_client.notify_hole_puncher(SocketAddr::new(IpAddr::from(Ipv4Addr::new(0,0,0,0)), 45000));
    udp_client.listen();

    //build_command();
}

fn register_ack_handler(_: SocketAddr, udp: &UdpSocket, message: &str) {
    println!("Got messag: {:?}", message);

     if message.replace("ACK_REGISTER | ", "") == "NO_PEER" {
         println!("No peer");
     } else {
        println!("Peer: {}", message.replace("ACK_REGISTER | ", ""));
        udp.send_to(b"PING |", message.replace("ACK_REGISTER | ", "").parse::<SocketAddr>().unwrap()).unwrap();
     }
}

fn ping_handler(source: SocketAddr, udp: &UdpSocket, _: &str) {
    println!("PING from {:?}", source.to_string());
    udp.send_to(b"PONG |", source).unwrap();
    println!("Send PONG");
}

fn pong_handler(source: SocketAddr, _: &UdpSocket, _: &str) {
    println!("PONG from {:?}", source.to_string());
}

fn build_command() {
    App::new("Blochain network cli")
        .version("0.1.0")
        .author("Lars Holznagel")
        .about("Client tool using the blockchain_network library")
        .arg(Arg::with_name("ip")
            .help("Sets the IP of the Hole puncher service")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("port")
            .help("Sets the port of the Hole puncher service.")
            .takes_value(true)
            .default_value("45000"))
        .get_matches();
}