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
//! For a config file example see the struct Config

extern crate base64;
#[macro_use]
extern crate failure;
extern crate yaml_rust;
extern crate sodiumoxide;

/// structs for the config files
pub mod config;
mod state;

/// Initialises the library
pub fn init(config: config::Config) {

    let _state = state::State::new(config);
}