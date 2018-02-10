#![deny(missing_docs)]

//! Hole puncher service
//!
//! Initiates a connection between two peers behind a NAT
//! Also saves all registered peers and provides them to new peers
//!
//! # Hole puncher
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
//!
//! Default port: 50000
extern crate blockchain_hooks;
#[macro_use]
extern crate blockchain_logging;
extern crate blockchain_protocol;

mod hooks;

use blockchain_hooks::{as_enum, HookRegister};

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

fn main() {
    info!("Starting hole puncher on port 50000");
    connect();
}

fn connect() {
    let state_handler = hooks::StateHandler::new();
    let mut hook_notification = HookRegister::new(Box::new(hooks::HookHandler), Arc::new(Mutex::new(state_handler)))
        .get_notification();

    let socket = UdpSocket::bind("0.0.0.0:50000").expect("Binding an UdpSocket should be successful.");

    loop {
        let mut buffer = [0; 65535];

        match socket.recv_from(&mut buffer) {
            Ok((bytes, source)) => {
                let mut updated_buffer = Vec::new();
                for i in 0..bytes {
                    updated_buffer.push(buffer[i])
                }

                let socket_clone = socket.try_clone().expect("Cloning the socket should be successful.");
                hook_notification.notify(socket_clone, as_enum(updated_buffer[0]), updated_buffer, source.to_string());
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}