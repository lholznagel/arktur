#![deny(missing_docs)]

//! Terminal client application
//!
//! Connects with the connection manager and to other peers
//! Calculates the hash value for a new block
extern crate blockchain_hooks;
#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_network;
extern crate blockchain_protocol;
extern crate clap;
extern crate crypto;

use blockchain_hooks::HookRegister;
use blockchain_network::udp_client::UdpClientBuilder;

use clap::{Arg, App};

use std::net::SocketAddr;

/// Contains all handlers the peer listens to
pub mod handlers;

fn main() {
    let matches = App::new("Blockchain network cli")
        .version("0.1.0")
        .author("Lars Holznagel")
        .about("Client tool using the blockchain_network library")
        .arg(Arg::with_name("HOLE_PUNCHER_IP")
            .value_name("ip")
            .help("Sets the IP of the Hole puncher service")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("PEER_NAME")
            .value_name("name")
            .help("Name of the peer")
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
    info!(format!("Hole puncher: {:?}", combined));
    connect(combined.parse::<SocketAddr>().unwrap(), String::from(matches.value_of("PEER_NAME").unwrap()));
}

/// Builds up a UDP connection with the connection manager
fn connect(addr: SocketAddr, name: String) {
    let hook_register = HookRegister::new()
        .add_hook(handlers::HookHandlers);

    let udp_client = UdpClientBuilder::new().build(hook_register);
    let udp_client = udp_client.notify_hole_puncher(addr, name);
    udp_client.listen();
}