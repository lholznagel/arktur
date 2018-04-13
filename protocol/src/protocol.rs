//! Contains the protocol model and a builder for the protocol
use errors::ParseErrors;
use nacl::Nacl;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{Nonce, PublicKey};
use payload::{Payload, parser};

/// temp solution
pub fn parse_encrypted(bytes: &[u8], nacl: &Nacl, public_key: &PublicKey) -> Result<Vec<u8>, ParseErrors> {
    let nonce = Nonce::from_slice(&bytes[0..24]).unwrap();
    Ok(box_::open(&bytes[24..], &nonce, &public_key, &nacl.get_secret_key()).unwrap())
}

/// Struct of the protocol
///
/// ```
/// // 00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 
/// //+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// //| Version               | Type                  |
/// //+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
/// //|                                               |
/// ////                                             //
/// ////                Payload                      //
/// ////                                             //
/// //|                                               |
/// //+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Protocol<T> {
    /// Identification of this message
    pub version: u8,
    /// Event that is fired, defined by a number between 0 and 255
    pub event_code: u8,
    /// Contains the content of the payload field
    pub payload: T
}

impl<T: Payload> Protocol<T> {
    /// Creates a new instance of the protocol information
    pub fn new() -> Self {

        Self {
            version: 1,
            event_code: 255,
            payload: T::new()
        }
    }

    /// Parses a byte array to the Protocol struct
    ///
    /// # Parameter
    ///
    /// - `payload` - byte array that should be parsed
    ///
    /// # Return
    ///
    /// Parsed result of the byte array
    pub fn from_bytes(payload: &[u8]) -> Result<Self, ParseErrors> {
        Protocol::parse(payload)
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

        let mut result = self.header_to_bytes();
        result.append(&mut self.payload.to_bytes());

        let encrypted = box_::seal(&result, &nonce, &public_key, &nacl.get_secret_key());
        payload.extend(encrypted);
        payload
    }

    /// Same as build but the payload is not encrypted using nacl
    /// 
    /// Should only be used for registering
    /// After sharing publickeys there is no reason to send
    /// non encrypted payloads!
    pub fn build_unencrypted(self, nacl: &mut Nacl) -> Vec<u8> {
        let nonce = nacl.get_nonce();
        let mut payload = Vec::new();
        payload.extend(nonce.0.iter());
        payload.extend(self.header_to_bytes());
        payload.extend(self.payload.to_bytes());
        payload
    }

    /// Parses the protocol
    ///
    /// # Parameters
    ///
    /// `bytes` - byte array of the protocol message
    ///
    /// # Return
    ///
    /// Protocol struct. See struct for more information
    fn parse(bytes: &[u8]) -> Result<Protocol<T>, ParseErrors> {
        let protocol = Protocol {
            version: bytes[0],
            event_code: bytes[1],
            payload: T::parse(parser::parse_payload(&bytes[2..])).unwrap()
        };

        Ok(protocol)
    }

    /// Turns the header values to bytes
    /// The checksum is excluded yfrom this. For that use `checksum_to_bytes()`
    ///
    /// # Return
    ///
    /// - `Vec<u8>` - Vector containing the header values as u8
    fn header_to_bytes(&self) -> Vec<u8> {
        vec![self.version.clone(), self.event_code.clone()]
    }
}