#![deny(missing_docs)]
#![feature(test)]

//! Helper library for parsing the protocol (still needs a better name) that is used in this project
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
extern crate test;

extern crate blockchain_hooks;
extern crate crc;
#[macro_use]
extern crate nom;
extern crate time;

pub mod enums;
pub mod payload;

mod protocol;
pub use protocol::BlockchainProtocol;