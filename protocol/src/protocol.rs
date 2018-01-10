//! Contains the protocol model and a builder for the protocol
use blockchain_hooks::{as_enum as as_enum_event, as_number as as_number_event, EventCodes};
use enums::status::{as_enum as as_enum_status, as_number as as_number_status, StatusCodes};
use payload::PayloadModel;
use nom::GetInput;
use std::{slice, mem};
use crc::crc32;

/// Parser for the protocol
named!(parse_protocol<&[u8], (u8, u8, u16, u16, u16, u32)>, bits!(tuple!(take_bits!(u8, 8), take_bits!(u8, 8), take_bits!(u16, 16), take_bits!(u16, 16), take_bits!(u16, 16), take_bits!(u32, 32))));

/// Parser error messages
#[derive(Debug)]
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
    /// Length of the added payload field
    pub payload_length: u16,
    /// Reserved for future use
    pub reserved: u16,
    /// Checksum of this message
    pub checksum: u32,
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
            payload_length: 0,
            reserved: 0,
            checksum: 0,
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
    ///         status_code: StatusCodes::Undefined,
    ///         id: 65535,
    ///         payload_length: 0,
    ///         reserved: 0,
    ///         checksum: 354351495,
    ///         payload: payload
    ///     };
    /// 
    ///     let payload = &[1, 255, 255, 255, 0, 0, 0, 0, 21, 30, 249, 135, 0];
    ///     let result = BlockchainProtocol::<PingPayload>::from_u8(payload);
    ///     assert_eq!(result.unwrap(), expected);
    /// # }
    /// ```
    pub fn from_vec(payload: Vec<u8>) -> Result<Self, ParseErrors> {
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
    ///         status_code: StatusCodes::Undefined,
    ///         id: 65535,
    ///         payload_length: 0,
    ///         reserved: 0,
    ///         checksum: 354351495,
    ///         payload: payload
    ///     };
    /// 
    ///     let payload = &[1, 255, 255, 255, 0, 0, 0, 0, 21, 30, 249, 135, 0];
    ///     let result = BlockchainProtocol::from_u8(payload);
    ///     assert_eq!(result.unwrap(), expected);
    /// # }
    /// ```
    pub fn from_u8(payload: &[u8]) -> Result<Self, ParseErrors> {
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
        let mut checksum = self.checksum_to_bytes(crc32::checksum_ieee(&self.header_to_bytes()));
        let mut result = self.header_to_bytes();
        result.append(&mut checksum);
        result.append(&mut self.payload.as_bytes());
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
    ///   /*  let payload = PingPayload::new();
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
    ///     let payload = &[1, 255, 255, 255, 0, 0, 0, 0, 21, 30, 249, 135, 0];
    ///     let result = BlockchainProtocol::from_u8(payload);
    ///     assert_eq!(result.unwrap(), expected);*/
    /// # }
    /// ```
    fn parse(bytes: &[u8]) -> Result<BlockchainProtocol<T>, ParseErrors> {
        let parsed = parse_protocol(bytes);
        let result = parsed.clone().to_result().unwrap();
        let remaining = BlockchainProtocol::<T>::parse_payload(parsed.remaining_input().unwrap());
        let payload = T::parse(remaining);

        let protocol = BlockchainProtocol {
            event_code: as_enum_event(result.0),
            status_code: as_enum_status(result.1),
            id: result.2,
            payload_length: result.3,
            reserved: result.4,
            checksum: result.5,
            payload: payload
        };

        if protocol.checksum == crc32::checksum_ieee(&protocol.header_to_bytes()) {
            Ok(protocol)
        } else {
            Err(ParseErrors::ChecksumDoNotMatch)
        }
    }

    /// Parses the payload
    ///
    /// # Parameters
    ///
    /// - `payload: Vec<u8>` - payload to parse
    ///
    /// # Returns
    ///
    /// - `Vec<Vec<u8>>` - Vector of vector containing the parsed payload
    fn parse_payload(payload: &[u8]) -> Vec<Vec<u8>> {
        let mut index: u64 = 0;
        let mut complete = Vec::new();

        if !payload.is_empty() {
            loop {
                if index == payload.len() as u64 {
                    break;
                }

                let mut current = Vec::new();
                let current_length = payload[index as usize];

                for i in (index + 1)..(index + current_length as u64 + 1) {
                    current.push(payload[i as usize]);
                    index += 1;
                }

                index += 1;
                complete.push(current);
            }
        }

        complete
    }

    /// Turns the header values to bytes
    /// The checksum is excluded from this. For that use `checksum_to_bytes()`
    ///
    /// # Return
    ///
    /// - `Vec<u8>` - Vector containing the header values as u8
    fn header_to_bytes(&self) -> Vec<u8> {
        let mut enums = vec![as_number_event(self.event_code.clone()), as_number_status(self.status_code.clone())];
        let slice_u16: &[u16] = &*vec![self.id, self.payload_length, self.reserved];
        let converted_slice: &[u8] = unsafe {
            slice::from_raw_parts(
                slice_u16.as_ptr() as *const u8,
                slice_u16.len() * mem::size_of::<u16>(),
            )
        };
        enums.append(&mut converted_slice.to_vec());
        enums
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

        vec![b1, b2, b3, b4]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockchain_hooks::EventCodes;
    use enums::status::StatusCodes;
    use payload::{PayloadModel, PingPayload};

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

        let payload = &[1, 255, 255, 255, 0, 0, 0, 0, 21, 30, 249, 135, 0];
        let result = BlockchainProtocol::<PingPayload>::from_u8(payload);
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

        let payload = &[0x01, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x15, 0x1E, 0xF9, 0x87, 0x00];
        let result = BlockchainProtocol::<PingPayload>::from_u8(payload);
        assert_eq!(result.unwrap(), expected);
    }

    // Needs to be redone as soon as all protocols use the new system
    /*#[test]
    fn test_with_payload() {
        let payload = RegisterAckPayload::new().set_addr(String::from("I am a test message"));
        let expected = BlockchainProtocol::<RegisterAckPayload> {
            event_code: EventCodes::Pong,
            status_code: StatusCodes::Undefined,
            id: 65535,
            payload_length: 23,
            reserved: 0,
            checksum: 5680,
            payload: payload,
        };

        let data = vec![1, 255, 255, 255, 22, 48, 0, 23, 126, 73, 32, 97, 109, 32, 97, 32, 116, 101, 115, 116, 32, 109, 101, 115, 115, 97, 103, 101, 126];
        let result = BlockchainProtocol::<RegisterAckPayload>::from_vec(data);
        assert_eq!(result.unwrap(), expected);
    }*/
}