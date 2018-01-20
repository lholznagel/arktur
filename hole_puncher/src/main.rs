#![deny(missing_docs)]

//! Hole puncher service
//!
//! Initiates a connection between two peers behind a NAT
//! Also saves all registered peers and provides them to new peers
//!
//! Default port: 50000
extern crate blockchain_hooks;
#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_network;
extern crate blockchain_protocol;

mod hooks;

use blockchain_hooks::HookRegister;
use blockchain_network::udp_client::UdpClientBuilder;

const UDP_PORT: u16 = 50000;

fn main() {
    info!("Starting hole puncher on port {}", UDP_PORT);
    connect();
}

fn connect() {
    let hook_handler = hooks::HookHandler::new();

    let hook_notification = HookRegister::new()
        .set_hook(hook_handler)
        .get_notification();

    UdpClientBuilder::new()
        .set_port(UDP_PORT)
        .build(hook_notification)
        .listen();
}