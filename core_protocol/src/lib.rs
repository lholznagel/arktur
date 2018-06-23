#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! This crate provides a wrapper for the protocol used for communicating
//! 
//! Let´s dive right in with a small ping example.
//! 
//! ```
//! extern crate carina_core_protocol;
//! extern crate sodiumoxide;
//! 
//! use carina_core_protocol::{Nacl, Payload, MessageBuilder};
//! use carina_core_protocol::payloads::EmptyPayload;
//! use sodiumoxide::crypto::box_;
//! use std::net::UdpSocket;
//! 
//! fn main() {
//!     // the public key comes from a config file
//!     let (therepk, _) = box_::gen_keypair();
//!     // our secret key also comes from a config file
//!     let (_, oursk) = box_::gen_keypair();
//!     // wrap out secret key a new struct
//!     // this struct also handles the nonce
//!     let mut nacl = Nacl::new(oursk);
//! 
//!     // create a new empty payload wit the event code 0 (ping)
//!     // in the build function we provide the nacl struct and the 
//!     // public key of the other peer
//!     let message = MessageBuilder::new()
//!         .set_event_code(0)
//!         .set_payload(carina_core_protocol::payloads::EmptyPayload::new())
//!         .build(&mut nacl, &therepk);
//! 
//!     // create a new udp socket
//!     let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
//!     // send the message
//!     socket.send_to(&message, "0.0.0.0:45000").unwrap();
//! }
//! ```

#[macro_use]
extern crate failure;
extern crate log;
extern crate protocol_builder_parser;
extern crate sodiumoxide;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate rand;

mod events;
mod nacl;
mod receive_message;
mod send_message_builder;

/// Module that contains all avaiable payloads
pub mod payloads;
/// Contains helper for events
pub use self::events::Events;
pub use self::payloads::Payload;
pub use self::nacl::Nacl;
pub use self::receive_message::decrypt;
pub use self::send_message_builder::MessageBuilder;