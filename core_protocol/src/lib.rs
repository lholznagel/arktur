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

//! Protocol for the core

extern crate failure;
extern crate log;
extern crate sodiumoxide;

mod nacl;
mod payload;
mod protocol;

/// Module that contains all avaiable payloads
pub mod payloads;
pub use self::nacl::Nacl;
pub use self::protocol::decrypt;