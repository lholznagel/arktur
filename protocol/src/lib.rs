#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! Library for creating and parsing the protocol used in the project
#[macro_use]
extern crate failure;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate sodiumoxide;

extern crate time;

pub mod payload;
pub mod errors;
pub mod nacl;

mod protocol;
pub use protocol::Protocol;
pub use errors::ParseErrors;
pub use protocol::parse_encrypted;