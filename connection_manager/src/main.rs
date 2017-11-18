#![deny(missing_docs)]

//! Manages connected peers
//!
//! - Handles hole punching between to peers
//! - Decides when and what comes into the next block (not yet implemented)
//! - Decides when a block is written (not yet implemented)

extern crate blockchain_file;
#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_network;
extern crate blockchain_protocol;

mod hole_puncher;
mod block_handler;

use blockchain_file::peers::KnownPeers;
use blockchain_network::event::EventHandler;
use blockchain_network::udp_client::UdpClientBuilder;

use std::thread;

/// Starting point
///
/// Registers all needed event handlers and starts a UDP-Listener
fn main() {
    KnownPeers::init();
    info!("Starting hole puncher!");

    let event_handlers = EventHandler::new()
        .set_register_handler(hole_puncher::register_handler);

    let udp = UdpClientBuilder::new()
        .set_port(45000)
        .build(event_handlers);

    info!("After udp");

    let local_udp = udp.connection();
    thread::spawn(move || {
        block_handler::handle_block(local_udp);
    });

    udp.listen();
}