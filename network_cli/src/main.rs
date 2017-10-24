extern crate blockchain_network;
extern crate clap;

use blockchain_network::udp_client::UdpClientBuilder;
use blockchain_network::event::EventHandler;

use clap::{Arg, App};

use std::net::{SocketAddr, UdpSocket};

fn main() {
    let matches = App::new("Blochain network cli")
        .version("0.1.0")
        .author("Lars Holznagel")
        .about("Client tool using the blockchain_network library")
        .arg(Arg::with_name("HOLE_PUNCHER_IP")
            .value_name("ip")
            .help("Sets the IP of the Hole puncher service")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("HOLE_PUNCHER_PORT")
            .value_name("port")
            .help("Sets the port of the Hole puncher service.")
            .takes_value(true)
            .default_value("45000"))
        .get_matches();

    let mut combined = String::from("");
    combined.push_str(matches.value_of("HOLE_PUNCHER_IP").unwrap());
    combined.push_str(":");
    combined.push_str(matches.value_of("HOLE_PUNCHER_PORT").unwrap());
    println!("{:?}", combined);
    connect(combined.parse::<SocketAddr>().unwrap());
}

fn connect(addr: SocketAddr) {
    let event_handler = EventHandler::new();
    let event_handler = event_handler
        .set_ping_handler(ping_handler)
        .set_pong_handler(pong_handler)
        .set_peer_registering_handler(peer_registering_handler)
        .set_register_ack_handler(register_ack_handler);

    let udp_client = UdpClientBuilder::new().build(event_handler);
    let udp_client = udp_client.notify_hole_puncher(addr);
    udp_client.listen();
}

fn ping_handler(source: SocketAddr, udp: &UdpSocket, _: &str) {
    println!("\x1B[0;36mEvent - PING from peer {:?} \x1B[0m", source.to_string());
    println!("\x1B[0;35mSending - PONG to peer {:?} \x1B[0m", source.to_string());
    udp.send_to(b"PONG |", source).unwrap();
    println!("\x1B[0;32mSuccessful - Send PONG\x1B[0m");
}

fn pong_handler(source: SocketAddr, _: &UdpSocket, _: &str) {
    println!("\x1B[0;36mEvent - PONG from peer {:?} \x1B[0m", source.to_string());
}

fn peer_registering_handler(_: SocketAddr, udp: &UdpSocket, message: &str) {
    println!("\x1B[0;36mEvent - PEER_REGISTERING {:?} \x1B[0m", message.replace("PEER_REGISTERING | ", ""));
    println!("\x1B[0;35mSending - PING to new peer {:?} \x1B[0m", message.replace("PEER_REGISTERING | ", ""));
    udp.send_to(b"PING |", message.replace("PEER_REGISTERING | ", "").parse::<SocketAddr>().unwrap()).unwrap();
    println!("\x1B[0;32mSuccessful - Send PING\x1B[0m");
}

fn register_ack_handler(_: SocketAddr, udp: &UdpSocket, message: &str) {
     if message.replace("ACK_REGISTER | ", "") == "NO_PEER" {
         println!("\x1B[0;32mNo peer\x1B[0m");
     } else {
        println!("\x1B[0;35mSending - PONG to peer {:?} \x1B[0m", message.replace("ACK_REGISTER | ", ""));
        udp.send_to(b"PING |", message.replace("ACK_REGISTER | ", "").parse::<SocketAddr>().unwrap()).unwrap();
        println!("\x1B[0;32mSuccessful - Send PING\x1B[0m");
     }
}