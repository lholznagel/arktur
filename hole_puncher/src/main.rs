#![deny(missing_docs)]

//! # What does it do?
//!
//! - Create a "hole" between to peers
//! - When a peer registers itself, its IP-Address + Port are saved
//! - The next peer that registers itself, gets these IP-Address + Port
//! - The older peer gets the IP-Address + Port of the new peer
//! - The address of the new peer are saved instead of the old peer
//! - Both start a ping event to the other peer
//! - With this technic a connection between two private networks can be accomplished
//!
//! In the following graphic, the process is shown
//!
//! ```
//!  1. Register  +--------------+ 2. Register
//!   +--------->|              |<---------+
//!   |          | hole puncher |          |
//!   |    +-----|              |-----+    |
//!   |    |     +--------------+     |    |
//!   |    | 3. Send IP+Port of new   |    |
//!   |    |                          |    |
//!   |    |                          |    |
//!   |    |                          |    |
//!   |    |   4. Send IP+Port of old |    |
//!   |    v                          v    |
//! +--------+                      +--------+
//! |        |--------------------->|        |
//! | Peer A |      5. Contact      | Peer B |
//! |        |<---------------------|        |
//! +--------+                      +--------+
//!
//! created with http://asciiflow.com/
//! ```
//!
//! # Example
//!
//! - Peer A runs on 192.168.1.5:45678 (on host a)
//! - Peer B runs on 192.168.1.6:56789 (on host b)
//! - Peer A registers itself at the hole puncher (some.public.ip.address:45000)
//! - The hole puncher does not know any peer
//! - Peer B registers itself at the same hole puncher
//! - The hole puncher sends the Peer B information to Peer A
//! - The hole puncher then sends the Peer A information to Peer B
//! - Peer A and Peer B try to ping each other
//! - The connection between both networks should be good to go

#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_network;
extern crate blockchain_protocol;

use blockchain_network::event::EventHandler;
use blockchain_network::udp_client::UdpClientBuilder;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::events::EventCodes;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{PayloadParser, PeerRegisteringPayload, RegisterPayload, RegisterAckPayload};

use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::net::{UdpSocket, SocketAddr};

/// Starting point
///
/// Registers all needed event handlers and starts a UDP-Listener
fn main() {
    remove_file("last_peer").unwrap();
    File::create("last_peer").unwrap();

    info!("Starting hole puncher!");

    let event_handlers = EventHandler::new().set_register_handler(register_handler);

    UdpClientBuilder::new()
        .set_port(45000)
        .build(event_handlers)
        .listen();
}

/// Handler for the REGISTER event
fn register_handler(source: SocketAddr, udp: &UdpSocket, _: BlockchainProtocol<RegisterPayload>) {
    let mut file = File::open("last_peer").unwrap();
    let mut content = String::from("");
    let mut status = StatusCodes::Ok;

    file.read_to_string(&mut content).unwrap();

    if content == "" {
        status = StatusCodes::NoPeer;
    } else {
        let payload = PeerRegisteringPayload::new().set_addr(source.to_string());
        let message = BlockchainProtocol::new()
            .set_event_code(EventCodes::PeerRegistering)
            .set_payload(payload)
            .build();
        udp.send_to(message.as_slice(), content.parse::<SocketAddr>().unwrap())
            .unwrap();
    }

    let mut file = File::create("last_peer").unwrap();
    file.write_all(source.to_string().as_bytes()).unwrap();

    let payload = RegisterAckPayload::new().set_addr(content);
    sending!(format!("ACK_REGISTER | {:?}", payload));
    let message = BlockchainProtocol::new()
        .set_event_code(EventCodes::AckRegister)
        .set_status_code(status)
        .set_payload(payload)
        .build();
    udp.send_to(message.as_slice(), source).unwrap();
}