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

//! Core implementation for a peer in the carina network
//! 
//! # Usage
//! ```
//! extern crate carina_core;
//! 
//! fn main() {
//!     let config_str = r#"---
//!         socket: /tmp/carina.sock
//!         peers: ./peers.yml
//!         storage: ./block_data
//!         uri: 0.0.0.0:45000
//!         secret_key: W8TAQuFECexfADKJik6WBrh4G5qFaOhzX2eBZFIV8kY="#;
//!     let config = carina_core::Config::from_str(config_str).unwrap();
//! 
//!     carina_core::init(config);
//! }
//! ```
extern crate base64;
#[macro_use]
extern crate failure;
extern crate yaml_rust;
extern crate sodiumoxide;

/// See the config file struct for more information
mod config;
mod state;

pub use config::Config;

/// Initialises the library
pub fn init(config: Config) {
    let _state = state::State::new(config);
}