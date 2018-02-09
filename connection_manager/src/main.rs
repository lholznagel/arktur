#![deny(missing_docs)]

//! Manages connected peers
//!
//! - Handles hole punching between to peers
//! - Decides when and what comes into the next block (not yet implemented)
//! - Decides when a block is written (not yet implemented)
extern crate blockchain_file;
extern crate blockchain_hooks;
#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_protocol;
extern crate time;

mod handlers;

use blockchain_file::peers::KnownPeers;
use blockchain_hooks::HookRegister;

/// Starting point
///
/// Registers all needed event handlers and starts a UDP-Listener
fn main() {
    KnownPeers::init();
    info!("Starting hole puncher!");

    /*let hook_handler = handlers::HookHandlers::new();

    let hook_notification = HookRegister::new()
        .set_hook(hook_handler)
        .get_notification();

    let udp = UdpClientBuilder::new()
        .set_port(45000)
        .build(hook_notification);

    udp.listen();*/
}