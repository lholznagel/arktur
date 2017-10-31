#![deny(missing_docs)]

//! Helper library for parsng the protocol (still needs a name) that is used in this project
#[macro_use]
extern crate nom;

pub mod enums;

mod protocol;

pub use protocol::BlockchainProtocol;