//! Contains the protocol model and a builder for the protocol
use blockchain_hooks::{as_enum as as_enum_event, as_number as as_number_event, EventCodes};
use enums::status::{as_enum as as_enum_status, as_number as as_number_status, StatusCodes};
use payload::PayloadModel;
use nom::GetInput;
use std::{slice, mem};

/// Parser for the protocol
named!(parse_protocol<&[u8], (u8, u8, u16, u16, u16)>, bits!(tuple!(take_bits!(u8, 8), take_bits!(u8, 8), take_bits!(u16, 16), take_bits!(u16, 16), take_bits!(u16, 16))));
named!(parse_delimited<Vec<&[u8]>>, many0!(delimited!(char!('~'), take_until!("~"), char!('~'))));

/// Struct of the protocol
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Event code            | Status                |                 ID                            |
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
/// - TODO: add checksum
#[derive(Clone, Debug, PartialEq)]
pub struct BlockchainProtocol<T> {
    /// Event that is fired, defined by a number between 0 and 255
    pub event_code: EventCodes,
    /// Status of this message, defined by a number between 0 and 255
    pub status_code: StatusCodes,
    /// Identification of this message
    pub id: u16,
    /// TTL of this message
    pub ttl: u16,
    /// Length of the added payload field
    pub payload_length: u16,
    /// Contains the content of the payload field
    pub payload: T,
}

impl<T: PayloadModel> BlockchainProtocol<T> {
    /// Creates a new instance of the protocol information
    pub fn new() -> Self {
        BlockchainProtocol {
            event_code: EventCodes::NotAValidEvent,
            status_code: StatusCodes::Undefined,
            id: 0,
            ttl: 0,
            payload_length: 0,
            payload: T::new(),
        }
    }

    /// Parses a vector to the BlockchainProtocol struct
    ///
    /// # Parameter
    ///
    /// - `payload` - byte vector that should be parsed
    ///
    /// # Return
    ///
    /// Parsed result from the vector
    ///
    /// # Example
    /// ```
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_protocol;
    ///
    /// use blockchain_hooks::EventCodes;
    /// use blockchain_protocol::BlockchainProtocol;
    /// use blockchain_protocol::enums::status::StatusCodes;
    /// use blockchain_protocol::payload::{PayloadModel, PingPayload};
    ///
    /// # fn main() {
    ///     let payload = PingPayload::new();
    ///     let expected = BlockchainProtocol {
    ///         event_code: EventCodes::Pong,
    ///         status_code: StatusCodes::Ok,
    ///         id: 65535,
    ///         ttl: 1337,
    ///         payload_length: 0,
    ///         payload: payload
    ///     };
    /// 
    ///     let payload = vec![1, 0, 255, 255, 5, 57, 0, 0];
    ///     let result = BlockchainProtocol::<PingPayload>::from_vec(payload);
    ///     assert_eq!(result, expected);
    /// # }
    /// ```
    pub fn from_vec(payload: Vec<u8>) -> Self {
        BlockchainProtocol::parse(payload.as_slice())
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
    /// use blockchain_protocol::payload::{PayloadModel, PingPayload};
    ///
    /// # fn main() {
    ///     let payload = PingPayload::new();
    ///     let expected = BlockchainProtocol {
    ///         event_code: EventCodes::Pong,
    ///         status_code: StatusCodes::Ok,
    ///         id: 65535,
    ///         ttl: 1337,
    ///         payload_length: 0,
    ///         payload: payload
    ///     };
    /// 
    ///     let payload = &[1, 0, 255, 255, 5, 57, 0, 0];
    ///     let result = BlockchainProtocol::from_u8(payload);
    ///     assert_eq!(result, expected);
    /// # }
    /// ```
    pub fn from_u8(payload: &[u8]) -> Self {
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
    pub fn build(self) -> Vec<u8> {
        let slice_u16: &[u16] = &*vec![self.id, self.ttl, self.payload.length()];
        let converted_slice: &[u8] = unsafe {
            slice::from_raw_parts(
                slice_u16.as_ptr() as *const u8,
                slice_u16.len() * mem::size_of::<u16>(),
            )
        };

        let mut result: Vec<u8> = Vec::new();
        result.push(as_number_event(self.event_code));
        result.push(as_number_status(self.status_code));
        result.push(converted_slice[0]);
        result.push(converted_slice[1]);
        result.push(converted_slice[2]);
        result.push(converted_slice[3]);
        result.push(converted_slice[4]);
        result.push(converted_slice[5]);

        let payload_converted = self.payload.as_bytes();
        for index in payload_converted {
            result.push(index);
        }

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
    /// use blockchain_protocol::payload::{PayloadModel, PingPayload};
    ///
    /// # fn main() {
    ///     let payload = PingPayload::new();
    ///     let expected = BlockchainProtocol {
    ///         event_code: EventCodes::Pong,
    ///         status_code: StatusCodes::Ok,
    ///         id: 65535,
    ///         ttl: 1337,
    ///         payload_length: 0,
    ///         payload: payload
    ///     };
    /// 
    ///     let payload = &[1, 0, 255, 255, 5, 57, 0, 0];
    ///     let result = BlockchainProtocol::from_u8(payload);
    ///     assert_eq!(result, expected);
    /// # }
    /// ```
    fn parse(bytes: &[u8]) -> BlockchainProtocol<T> {
        let parsed = parse_protocol(bytes);
        let result = parsed.clone().to_result().unwrap();
        let remaining = parse_delimited(parsed.remaining_input().unwrap())
            .to_result()
            .unwrap();
        let payload = T::parse(remaining);

        BlockchainProtocol {
            event_code: as_enum_event(result.0),
            status_code: as_enum_status(result.1),
            id: result.2,
            ttl: result.3,
            payload_length: result.4,
            payload: payload
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockchain_hooks::EventCodes;
    use enums::status::StatusCodes;
    use payload::{PayloadModel, PingPayload, RegisterAckPayload};

    #[test]
    fn test_u8() {
        let payload = PingPayload::new();
        let expected = BlockchainProtocol::<PingPayload> {
            event_code: EventCodes::Pong,
            status_code: StatusCodes::Undefined,
            id: 65535,
            ttl: 1337,
            payload_length: 0,
            payload: payload,
        };

        let payload = &[1, 255, 255, 255, 5, 57, 0, 0];
        let result = BlockchainProtocol::<PingPayload>::from_u8(payload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hex() {
        let payload = PingPayload::new();
        let expected = BlockchainProtocol::<PingPayload> {
            event_code: EventCodes::Pong,
            status_code: StatusCodes::Undefined,
            id: 65535,
            ttl: 1337,
            payload_length: 0,
            payload: payload,
        };

        let payload = &[0x01, 0xFF, 0xFF, 0xFF, 0x05, 0x39, 0x00, 0x00];
        let result = BlockchainProtocol::<PingPayload>::from_u8(payload);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_payload() {
        let payload = RegisterAckPayload::new().set_addr(String::from("I am a test message"));
        let expected = BlockchainProtocol::<RegisterAckPayload> {
            event_code: EventCodes::Pong,
            status_code: StatusCodes::Undefined,
            id: 65535,
            ttl: 1337,
            payload_length: 23,
            payload: payload,
        };

        let data = vec![1, 255, 255, 255, 5, 57, 0, 23, 126, 73, 32, 97, 109, 32, 97, 32, 116, 101, 115, 116, 32, 109, 101, 115, 115, 97, 103, 101, 126];
        let result = BlockchainProtocol::<RegisterAckPayload>::from_vec(data);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_empty_payload() {
        let result = parse_delimited(&[126, 116, 101, 115, 116, 126, 126, 126])
            .to_result()
            .unwrap();

        assert_eq!(result.len(), 2);
    }
}