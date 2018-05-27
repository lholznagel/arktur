//! Contains the protocol model and a builder for the protocol
use nacl::Nacl;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{Nonce, PublicKey};

/// temp solution
pub fn decrypt(bytes: &[u8], nacl: &Nacl, public_key: &PublicKey) -> Vec<u8> {
    let nonce = Nonce::from_slice(&bytes[0..24]).unwrap();
    match box_::open(&bytes[24..], &nonce, &public_key, &nacl.get_secret_key()) {
        Ok(val) => val,
        Err(_) => bytes[24..].to_vec()
    }
}