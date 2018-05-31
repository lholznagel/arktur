//! Contains the protocol model and a builder for the protocol
use nacl::Nacl;
use payloads::Payload;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey;

///This struct represents the structure of the protocol
#[derive(Clone, Debug, PartialEq)]
pub struct MessageBuilder<T> {
    /// protocol version that is used. until there is a new version always 1
    pub version: u8,
    /// Event code of the payload. Value between 0 and 255
    pub event_code: u8,
    /// the actual payload
    pub payload: T,
}

impl<T: Payload> MessageBuilder<T> {
    /// Creates a new protocol builder instance
    pub fn new() -> Self {
        Self {
            version: 1,
            event_code: 255,
            payload: T::new(),
        }
    }

    /// Sets the event code
    pub fn set_event_code(mut self, event_code: u8) -> Self {
        self.event_code = event_code;
        self
    }

    /// sets the payload
    /// 
    /// The payload must be something that implements the trait `Payload`
    pub fn set_payload(mut self, payload: T) -> Self {
        self.payload = payload;
        self
    }

    /// Combines all protocol information together and converts them to a
    /// sendable `Vec<u8>`
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
