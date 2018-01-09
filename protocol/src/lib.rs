#![deny(missing_docs)]

//! Helper library for parsing the protocol (still needs a better name) that is used in this project

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate crc;

extern crate blockchain_hooks;
#[macro_use]
extern crate nom;
extern crate time;

pub mod enums;
pub mod payload;

mod protocol;
pub use protocol::BlockchainProtocol;