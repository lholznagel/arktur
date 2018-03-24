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

//! Helper library for parsing the protocol (still needs a better name) that is used in this project
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate crc;
extern crate time;

pub mod payload;
pub mod errors;

mod protocol;
pub use protocol::Protocol;
pub use errors::ParseErrors;