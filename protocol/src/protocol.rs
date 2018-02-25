//! Contains the protocol model and a builder for the protocol
use blockchain_hooks::{as_enum as as_enum_event, as_number as as_number_event, EventCodes};
use enums::status::{as_enum as as_enum_status, as_number as as_number_status, StatusCodes};
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
/// // | Event code            | Status                |                 ID                            |
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
    pub event_code: EventCodes,
    /// Status of this message, defined by a number between 0 and 255
    pub status_code: StatusCodes,
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
            event_code: EventCodes::NotAValidEvent,
            status_code: StatusCodes::Undefined,
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
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_protocol;
    ///
    /// use blockchain_hooks::EventCodes;
    /// use blockchain_protocol::BlockchainProtocol;
    /// use blockchain_protocol::enums::status::StatusCodes;
    /// use blockchain_protocol::payload::{Payload, PingPayload};
    ///
    /// # fn main() {
    ///     let payload = PingPayload::new();
    ///     let expected = BlockchainProtocol {
    ///         event_code: EventCodes::Pong,
    ///         status_code: StatusCodes::Undefined,
    ///         id: 65535,
    ///         payload_length: 0,
    ///         reserved: 0,
    ///         checksum: 354351495,
    ///         payload: payload
    ///     };
    /// 
    ///     let payload = &[1, 255, 255, 255, 0, 0, 0, 0, 135, 249, 30, 21, 0];
    ///     let result = BlockchainProtocol::from_bytes(payload);
    ///     assert_eq!(result.unwrap(), expected);
    /// # }
    /// ```
    pub fn from_bytes(payload: &[u8]) -> Result<Self, ParseErrors> {
        BlockchainProtocol::parse(payload)
    }

    /// Sets the event code
    ///
    /// # Default
    ///
    /// EventCodes::NotAValidEvent
    ///
    /// # Parameters
    ///
    /// - `event_code` - Event code
    ///
    /// # Return
    ///
    /// Updated instance of the struct
    pub fn set_event_code(mut self, event_code: EventCodes) -> Self {
        self.event_code = event_code;
        self
    }

    /// Sets the status code
    ///
    /// # Default
    ///
    /// StatusCodes::Undefined
    ///
    /// # Parameters
    ///
    /// - `status_code` - status code of the message
    ///
    /// # Return
    ///
    /// Updated instance of the struct
    pub fn set_status_code(mut self, status_code: StatusCodes) -> Self {
        self.status_code = status_code;
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
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_protocol;
    ///
    /// use blockchain_hooks::EventCodes;
    /// use blockchain_protocol::BlockchainProtocol;
    /// use blockchain_protocol::enums::status::StatusCodes;
    /// use blockchain_protocol::payload::{Payload, PingPayload};
    ///
    /// # fn main() {
    ///     let payload = PingPayload::new();
    ///     let expected = BlockchainProtocol {
    ///         event_code: EventCodes::Pong,
    ///         status_code: StatusCodes::Undefined,
    ///         id: 65535,
    ///         payload_length: 0,
    ///         checksum: 354351495,
    ///         reserved: 0,
    ///         payload: payload
    ///     };
    /// 
    ///     let payload = &[1, 255, 255, 255, 0, 0, 0, 0, 135, 249, 30, 21, 0];
    ///     let result = BlockchainProtocol::from_bytes(payload);
    ///     assert_eq!(result.unwrap(), expected);
    /// # }
    /// ```
    fn parse(bytes: &[u8]) -> Result<BlockchainProtocol<T>, ParseErrors> {
        let protocol = BlockchainProtocol {
            event_code: as_enum_event(bytes[0]),
            status_code: as_enum_status(bytes[1]),
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
        let mut result = vec![as_number_event(self.event_code.clone()), as_number_status(self.status_code.clone())];
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
    use blockchain_hooks::EventCodes;
    use enums::status::StatusCodes;
    use payload::{Payload, PingPayload};

    #[test]
    fn test_u8() {
        let payload = PingPayload::new();
        let expected = BlockchainProtocol::<PingPayload> {
            event_code: EventCodes::Pong,
            status_code: StatusCodes::Undefined,
            id: 65535,
            payload_length: 0,
            reserved: 0,
            checksum: 354351495,
            payload: payload,
        };

        let payload = &[1, 255, 255, 255, 0, 0, 0, 0, 135, 249, 30, 21, 0];
        let result = BlockchainProtocol::<PingPayload>::from_bytes(payload);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_hex() {
        let payload = PingPayload::new();
        let expected = BlockchainProtocol::<PingPayload> {
            event_code: EventCodes::Pong,
            status_code: StatusCodes::Undefined,
            id: 65535,
            payload_length: 0,
            reserved: 0,
            checksum: 354351495,
            payload: payload,
        };

        let payload = &[0x01, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x87, 0xF9, 0x1E, 0x15, 0x00];
        let result = BlockchainProtocol::<PingPayload>::from_bytes(payload);
        assert_eq!(result.unwrap(), expected);
    }
}