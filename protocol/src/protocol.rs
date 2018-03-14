//! Contains the protocol model and a builder for the protocol
use payload::{Parser, Payload};
use std::{slice, mem};
use crc::crc32;

/// Parser error messages
#[derive(Copy, Clone, Debug)]
pub enum ParseErrors {
    ChecksumDoNotMatch
}

/// Struct of the protocol
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Event code            |                       |                 ID                            |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // |               Data length                     |                 Reserved                      |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Checksum                                                                                      |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // |                                                                                               |
/// // //                                                                                             //
/// // //                Data                                                                         //
/// // //                                                                                             //
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct BlockchainProtocol<T> {
    /// Event that is fired, defined by a number between 0 and 255
    pub event_code: u8,
    /// Identification of this message
    pub id: u16,
    /// Identification of this message
    pub payload_length: u16,
    /// Reserved for future use
    pub reserved: u16,
    /// Checksum of this message
    pub checksum: u32,
    /// Contains the content of the payload field
    pub payload: T,
}

impl<T: Payload> BlockchainProtocol<T> {
    /// Creates a new instance of the protocol information
    pub fn new() -> Self {
        BlockchainProtocol {
            event_code: 255,
            id: 0,
            payload_length: 0,
            reserved: 0,
            checksum: 0,
            payload: T::new(),
        }
    }

    /// Parses a byte array to the BlockchainProtocol struct
    ///
    /// # Parameter
    ///
    /// - `payload` - byte array that should be parsed
    ///
    /// # Return
    ///
    /// Parsed result of the byte array
    ///
    /// # Example
    /// ```
    /// extern crate blockchain_protocol;
    ///
    /// use blockchain_protocol::BlockchainProtocol;
    /// use blockchain_protocol::payload::{EmptyPayload, Payload};
    ///
    /// # fn main() {
    ///     let payload = EmptyPayload::new();
    ///     let expected = BlockchainProtocol {
    ///         event_code: 1,
    ///         id: 65535,
    ///         payload_length: 1,
    ///         reserved: 0,
    ///         checksum: 1066718418,
    ///         payload: payload
    ///     };
    /// 
    ///     let payload = &[1, 0, 255, 255, 1, 0, 0, 0, 210, 212, 148, 63, 0];
    ///     let result = BlockchainProtocol::from_bytes(payload);
    ///     assert_eq!(result.unwrap(), expected);
    /// # }
    /// ```
    pub fn from_bytes(payload: &[u8]) -> Result<Self, ParseErrors> {
        BlockchainProtocol::parse(payload)
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
    pub fn build(mut self) -> Vec<u8> {
        self.payload_length = self.payload.clone().to_bytes().len() as u16;
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
    /// BlockchainProtocol struct. See struct for more information
    ///
    /// # Example
    /// ```
    /// extern crate blockchain_protocol;
    ///
    /// use blockchain_protocol::BlockchainProtocol;
    /// use blockchain_protocol::payload::{EmptyPayload, Payload};
    ///
    /// # fn main() {
    ///     let payload = EmptyPayload::new();
    ///     let expected = BlockchainProtocol {
    ///         event_code: 1,
    ///         id: 65535,
    ///         payload_length: 1,
    ///         checksum: 1066718418,
    ///         reserved: 0,
    ///         payload: payload
    ///     };
    /// 
    ///     let payload = &[1, 0, 255, 255, 1, 0, 0, 0, 210, 212, 148, 63, 0];
    ///     let result = BlockchainProtocol::from_bytes(payload);
    ///     assert_eq!(result.unwrap(), expected);
    /// # }
    /// ```
    fn parse(bytes: &[u8]) -> Result<BlockchainProtocol<T>, ParseErrors> {
        let protocol = BlockchainProtocol {
            event_code: bytes[0],
            id: Parser::u8_to_u16(&bytes[2..4]),
            payload_length: Parser::u8_to_u16(&bytes[4..6]),
            reserved: 0,
            checksum: Parser::u8_to_u32(&bytes[8..12]),
            payload: T::parse(Parser::parse_payload(&bytes[12..]))
        };

        if protocol.checksum == crc32::checksum_ieee(&protocol.header_to_bytes()) {
            Ok(protocol)
        } else {
            Err(ParseErrors::ChecksumDoNotMatch)
        }
    }

    /// Turns the header values to bytes
    /// The checksum is excluded from this. For that use `checksum_to_bytes()`
    ///
    /// # Return
    ///
    /// - `Vec<u8>` - Vector containing the header values as u8
    fn header_to_bytes(&self) -> Vec<u8> {
        let mut result = vec![self.event_code.clone(), 0];
        let slice_u16: &[u16] = &*vec![self.id, self.payload_length, self.reserved];
        let converted_slice: &[u8] = unsafe {
            slice::from_raw_parts(
                slice_u16.as_ptr() as *const u8,
                slice_u16.len() * mem::size_of::<u16>(),
            )
        };
        result.append(&mut converted_slice.to_vec());
        result
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

#[cfg(test)]
mod tests {
    use super::*;
    use payload::{Payload, EmptyPayload};

    #[test]
    fn test_u8() {
        let payload = EmptyPayload::new();
        let expected = BlockchainProtocol::<EmptyPayload> {
            event_code: 1,
            id: 65535,
            payload_length: 1,
            reserved: 0,
            checksum: 1066718418,
            payload: payload,
        };

        let payload = &[1, 0, 255, 255, 1, 0, 0, 0, 210, 212, 148, 63, 0];
        let result = BlockchainProtocol::<EmptyPayload>::from_bytes(payload);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_hex() {
        let payload = EmptyPayload::new();
        let expected = BlockchainProtocol::<EmptyPayload> {
            event_code: 1,
            id: 65535,
            payload_length: 1,
            reserved: 0,
            checksum: 1066718418,
            payload: payload,
        };

        let payload = &[0x01, 0x00, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0xD2, 0xD4, 0x94, 0x3F, 0x00];
        let result = BlockchainProtocol::<EmptyPayload>::from_bytes(payload);
        assert_eq!(result.unwrap(), expected);
    }
}