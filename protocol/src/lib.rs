//#![deny(missing_docs)]

//! Helper library for parsing the protocol (still needs a better name) that is used in this project
#[macro_use]
extern crate nom;

pub mod enums;
pub mod payload;

mod protocol;

pub use protocol::BlockchainProtocol;