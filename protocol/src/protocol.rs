//! Contains the protocol model and a builder for the protocol
use errors::ParseErrors;
use nacl::Nacl;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{Nonce, PublicKey};
use payload::{Payload, parser};
use crc::crc32;

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
/// //| Checksum                                      |
/// //|                                               |
/// //+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
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
    /// Checksum of this message
    pub checksum: u32,
    /// Contains the content of the payload field
    pub payload: T
}

impl<T: Payload> Protocol<T> {
    /// Creates a new instance of the protocol information
    pub fn new() -> Self {

        Self {
            version: 1,
            event_code: 255,
            checksum: 0,
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
    pub fn from_bytes(payload: &[u8], nacl: &Nacl, public_key: &PublicKey) -> Result<Self, ParseErrors> {
        Protocol::parse(payload, nacl, public_key)
    }

    /// Same as from_bytes but unencrypted
    /// TODO: nonce
    pub fn from_bytes_unencrypted(payload: &[u8]) -> Result<Self, ParseErrors> {
        Protocol::parse_unencrypted(payload)
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

        let mut checksum = self.checksum_to_bytes(crc32::checksum_ieee(&self.header_to_bytes()));
        let mut result = self.header_to_bytes();
        result.append(&mut checksum);
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
    /// TODO: parse nonce
    pub fn build_unencrypted(self) -> Vec<u8> {
        let mut checksum = self.checksum_to_bytes(crc32::checksum_ieee(&self.header_to_bytes()));
        let mut result = self.header_to_bytes();
        result.append(&mut checksum);
        result.append(&mut self.payload.to_bytes());
        result
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
    fn parse(bytes: &[u8], _nacl: &Nacl, _public_key: &PublicKey) -> Result<Protocol<T>, ParseErrors> {
        let protocol = Protocol {
            version: bytes[0],
            event_code: bytes[1],
            checksum: parser::u8_to_u32(&bytes[2..6])?,
            payload: T::parse(parser::parse_payload(&bytes[6..])).unwrap()
        };

        if protocol.checksum == crc32::checksum_ieee(&protocol.header_to_bytes()) {
            Ok(protocol)
        } else {
            Err(ParseErrors::ChecksumDoNotMatch)
        }
    }

    /// Same as parse but without encryption
    /// TODO: parse nonce
    fn parse_unencrypted(bytes: &[u8]) -> Result<Protocol<T>, ParseErrors> {
        let protocol = Protocol {
            version: bytes[0],
            event_code: bytes[1],
            checksum: parser::u8_to_u32(&bytes[2..6])?,
            payload: T::parse(parser::parse_payload(&bytes[6..]))?
        };

        if protocol.checksum == crc32::checksum_ieee(&protocol.header_to_bytes()) {
            Ok(protocol)
        } else {
            Err(ParseErrors::ChecksumDoNotMatch)
        }
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

    /// Turns the checksum bytes
    ///
    /// # Params
    ///
    /// - `checksum` - checksum that should be converted
    ///
    /// # Return
    ///
    /// - `Vec<u8>` - Vector containing the header values as u8
    fn checksum_to_bytes(&self, checksum: u32) -> Vec<u8> {
        let b1 = ((checksum >> 24) & 0xFF) as u8;
        let b2 = ((checksum >> 16) & 0xFF) as u8;
        let b3 = ((checksum >> 8) & 0xFF) as u8;
        let b4 = (checksum & 0xFF) as u8;

        vec![b4, b3, b2, b1]
    }
}