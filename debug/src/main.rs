#![deny(missing_docs)]

//! Small debug application
extern crate blockchain_hooks;
#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_network;
extern crate blockchain_protocol;

use blockchain_hooks::{EventCodes, HookRegister};
use blockchain_network::udp_client::UdpClientBuilder;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::{ExploreNetworkPayload, Payload};
use blockchain_protocol::enums::status::StatusCodes;

use std::net::SocketAddr;

/// Contains all handlers the peer listens to
pub mod handlers;

fn main() {
    let hook_notification = HookRegister::new()
        .set_hook(handlers::HookHandler::new())
        .get_notification();

    let udp_client = UdpClientBuilder::new().build(hook_notification);

    let request = BlockchainProtocol::<ExploreNetworkPayload>::new()
            .set_event_code(EventCodes::ExploreNetwork)
            .set_status_code(StatusCodes::Ok)
            .build();
    udp_client.udp.send_to(&request, "0.0.0.0:50000".parse::<SocketAddr>().unwrap()).expect("Sending a request should be successful");

    udp_client.listen();
}