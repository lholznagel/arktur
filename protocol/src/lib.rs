#![deny(missing_docs)]

//! Helper library for parsng the protocol (still needs a name) that is used in this project
#[macro_use]
extern crate nom;

pub mod hex;

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
    pub event_code: u8,
    /// Status of this message, defined by a number between 0 and 255
    pub status_code: u8,
    /// Identification of this message
    pub id: u16,
    /// TTL of this message
    pub ttl: u16,
    /// Length of the added data field
    pub data_length: u16
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
///
/// let expected = BlockchainProtocol {
///     event_code: 1,
///     status_code: 2,
///     id: 65535,
///     ttl: 1337,
///     data_length: 0
/// };
///
/// let data = &[1, 2, 255, 255, 5, 57, 0, 0];
/// let result = parse(data);
/// assert_eq!(result, expected);
/// ```
pub fn parse(bytes: &[u8]) -> BlockchainProtocol {
    let parsed = parse_protocol(bytes).to_result().unwrap();

    BlockchainProtocol {
        event_code: parsed.0,
        status_code: parsed.1,
        id: parsed.2,
        ttl: parsed.3,
        data_length: parsed.4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::{FromHex, ToHex};

    #[test]
    fn test_simple_u8() {
        let expected = BlockchainProtocol {
            event_code: 1,
            status_code: 2,
            id: 65535,
            ttl: 1337,
            data_length: 0
        };

        let data = &[1, 2, 255, 255, 5, 57, 0, 0];
        let result = parse(data);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_hex() {
        let expected = BlockchainProtocol {
            event_code: 1,
            status_code: 2,
            id: 65535,
            ttl: 1337,
            data_length: 0
        };

        let data = &[0x01, 0x02, 0xFF, 0xFF, 0x05, 0x39, 0x00, 0x00];
        let result = parse(data);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_string_u8() {
        let expected = BlockchainProtocol {
            event_code: 1,
            status_code: 2,
            id: 65535,
            ttl: 1337,
            data_length: 0
        };

        let data = &[1, 2, 255, 255, 5, 57, 0, 0];
        let data = data.to_hex();
        let data = data.as_str().from_hex();
        let result = parse(data.as_slice());
        assert_eq!(result, expected);
    }
}