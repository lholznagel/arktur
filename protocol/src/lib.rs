#![deny(missing_docs)]

//! Helper library for parsng the protocol (still needs a name) that is used in this project
#[macro_use]
extern crate nom;

pub mod hex;
pub mod enums;

use enums::events::{as_enum, EventCodes};
use nom::GetInput;
use std::str;

/// Parser for the protocol
named!(parse_protocol<&[u8], (u8, u8, u16, u16, u16)>, bits!(tuple!(take_bits!(u8, 8), take_bits!(u8, 8), take_bits!(u16, 16), take_bits!(u16, 16), take_bits!(u16, 16))));

/// Struct of the protocol
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Eventcode             | Status                |                 ID                            |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // |                 TTL                           |                 Data length                   |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // |                                                                                               |
/// // //                                                                                             //
/// // //                Data                                                                         //
/// // //                                                                                             //
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
///
/// - TODO: implement Data field
/// - TODO: add checksum
#[derive(Debug, PartialEq)]
pub struct BlockchainProtocol {
    /// Event that is fired, defined by a number between 0 and 255
    pub event_code: EventCodes,
    /// Status of this message, defined by a number between 0 and 255
    pub status_code: u8,
    /// Identification of this message
    pub id: u16,
    /// TTL of this message
    pub ttl: u16,
    /// Length of the added data field
    pub data_length: u16,
    /// Contains the content of the data field
    pub data: String,
}

/// Parses the protocol
///
/// # Parameters
///
/// `bytes` - byte array of the protocol message
///
/// # Return
///
/// BLockchainProtocl struct. See struct for more information
///
/// # Example
/// ```
/// use blockchain_protocol::{BlockchainProtocol, parse};
/// use blockchain_protocol::enums::events::EventCodes;
///
/// let expected = BlockchainProtocol {
///     event_code: EventCodes::Pong,
///     status_code: 2,
///     id: 65535,
///     ttl: 1337,
///     data_length: 0,
///     data: String::from("")
/// };
///
/// let data = &[1, 2, 255, 255, 5, 57, 0, 0];
/// let result = parse(data);
/// assert_eq!(result, expected);
/// ```
pub fn parse(bytes: &[u8]) -> BlockchainProtocol {
    let parsed = parse_protocol(bytes);
    let result = parsed.clone().to_result().unwrap();
    let remaining = parsed.remaining_input().unwrap();

    BlockchainProtocol {
        event_code: as_enum(result.0),
        status_code: result.1,
        id: result.2,
        ttl: result.3,
        data_length: result.4,
        data: str::from_utf8(remaining).unwrap().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enums::events::EventCodes;
    use hex::{FromHex, ToHex};

    #[test]
    fn test_u8() {
        let expected = BlockchainProtocol {
            event_code: EventCodes::Pong,
            status_code: 2,
            id: 65535,
            ttl: 1337,
            data_length: 0,
            data: String::from(""),
        };

        let data = &[1, 2, 255, 255, 5, 57, 0, 0];
        let result = parse(data);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hex() {
        let expected = BlockchainProtocol {
            event_code: EventCodes::Pong,
            status_code: 2,
            id: 65535,
            ttl: 1337,
            data_length: 0,
            data: String::from(""),
        };

        let data = &[0x01, 0x02, 0xFF, 0xFF, 0x05, 0x39, 0x00, 0x00];
        let result = parse(data);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_u8() {
        let expected = BlockchainProtocol {
            event_code: EventCodes::Pong,
            status_code: 2,
            id: 65535,
            ttl: 1337,
            data_length: 0,
            data: String::from(""),
        };

        let data = &[1, 2, 255, 255, 5, 57, 0, 0];
        let data = data.to_hex();
        let data = data.as_str().from_hex();
        let result = parse(data.as_slice());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_data() {
        let expected = BlockchainProtocol {
            event_code: EventCodes::Pong,
            status_code: 2,
            id: 65535,
            ttl: 1337,
            data_length: 0,
            data: String::from("I am a test message"),
        };

        let data = &[1, 2, 255, 255, 5, 57, 0, 0, 73, 32, 97, 109, 32, 97, 32, 116, 101, 115, 116, 32, 109, 101, 115, 115, 97, 103, 101];
        let result = parse(data);
        assert_eq!(result, expected);
    }
}