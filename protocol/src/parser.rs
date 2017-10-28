//! Contains everything that is need to parse the protocol
use enums::events::as_enum;
use nom::GetInput;
use protocol::BlockchainProtocol;
use std::str;

/// Parser for the protocol
named!(parse_protocol<&[u8], (u8, u8, u16, u16, u16)>, bits!(tuple!(take_bits!(u8, 8), take_bits!(u8, 8), take_bits!(u16, 16), take_bits!(u16, 16), take_bits!(u16, 16))));

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