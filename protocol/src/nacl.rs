//! Struct for representing nacl

use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{Nonce, PublicKey, SecretKey};

/// Holds the public key, secret key and the nonce
#[derive(Clone, Debug)]
pub struct Nacl {
    public_key: PublicKey,
    secret_key: SecretKey,
    nonce: Nonce
}

impl Nacl {
    /// Creates a new instance
    pub fn new() -> Self {
        let (public_key, secret_key) = box_::gen_keypair();
        let nonce = box_::gen_nonce();

        Self {
            public_key,
            secret_key,
            nonce
        }
    }

    /// Gets the current nonce and increments it
    pub fn get_nonce(&mut self) -> Nonce {
        self.nonce.increment_le()
    }

    /// Gets the public key
    pub fn get_public_key(&self) -> PublicKey {
        self.public_key
    }

    /// Gets the secret key
    pub(crate) fn get_secret_key(&self) -> SecretKey {
        self.secret_key.clone()
    }
}