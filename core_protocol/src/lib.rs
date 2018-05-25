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

extern crate log;
extern crate sodiumoxide;

use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{Nonce, PublicKey, SecretKey};

/// Decryptes the given message
pub fn parse_encrypted(bytes: &[u8], secret_key: &SecretKey, public_key: &PublicKey) -> Vec<u8> {
    let nonce = Nonce::from_slice(&bytes[0..24]).unwrap();
    match box_::open(&bytes[24..], &nonce, &public_key, &secret_key) {
        Ok(val) => val,
        Err(_) => bytes[24..].to_vec()
    }
}