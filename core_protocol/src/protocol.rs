//! Contains the protocol model and a builder for the protocol
use nacl::Nacl;
use payloads::Payload;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{Nonce, PublicKey};

/// temp solution
pub fn decrypt(bytes: &[u8], nacl: &Nacl, public_key: &PublicKey) -> Vec<u8> {
    let nonce = Nonce::from_slice(&bytes[0..24]).unwrap();
    match box_::open(&bytes[24..], &nonce, &public_key, &nacl.get_secret_key()) {
        Ok(val) => val,
        Err(_) => bytes[24..].to_vec(),
    }
}

/// Holds all important information for a new protocol message
#[derive(Clone, Debug, PartialEq)]
pub struct Protocol<T> {
    /// Identification of this message
    pub version: u8,
    /// Event that is fired, defined by a number between 0 and 255
    pub event_code: u8,
    /// Contains the content of the payload field
    pub payload: T,
}

impl<T: Payload> Protocol<T> {
    /// Creates a new protocol instance
    pub fn new() -> Self {
        Self {
            version: 1,
            event_code: 255,
            payload: T::new(),
        }
    }

    /// Sets the event code
    ///
    /// # Parameters
    ///
    /// - `event_code` - Event code
    ///
    /// # Return
    ///
    /// Updated instance of the struct
    pub fn set_event_code(mut self, event_code: u8) -> Self {
        self.event_code = event_code;
        self
    }

    /// Sets the payload that should be send as payload
    ///
    /// # Default
    ///
    /// Empty String
    ///
    /// # Parameters
    ///
    /// - `payload` - payload that should be send
    ///
    /// # Return
    ///
    /// Updated instance of the struct
    pub fn set_payload(mut self, payload: T) -> Self {
        self.payload = payload;
        self
    }

    /// Combines the struct to a vector of bytes
    pub fn build(self, nacl: &mut Nacl, public_key: &PublicKey) -> Vec<u8> {
        let nonce = nacl.get_nonce();
        let mut payload = Vec::new();
        payload.extend(nonce.0.iter());

        let mut result = vec![self.version.clone(), self.event_code.clone()];
        result.append(&mut self.payload.to_bytes());

        let encrypted = box_::seal(&result, &nonce, &public_key, &nacl.get_secret_key());
        payload.extend(encrypted);
        payload
    }
}
