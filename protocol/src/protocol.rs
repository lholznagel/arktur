//! Contains the protocol model and a builder for the protocol
use enums::events::{as_number, EventCodes};
use std::{slice, mem};
use parser::parse;
use hex::FromHex;

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

impl BlockchainProtocol {
    /// Creates a new instance of the protocol information
    pub fn new() -> Self {
        BlockchainProtocol {
            event_code: EventCodes::NotAValidEvent,
            status_code: 0,
            id: 0,
            ttl: 0,
            data_length: 0,
            data: String::from(""),
        }
    }

    /// Parses a vector to the BlockchainProtocol struct
    ///
    /// # Parameter
    ///
    /// - `data` - byte vector that should be parsed
    ///
    /// # Return
    ///
    /// Parsed result from the vector
    ///
    /// TODO: example
    pub fn from_vec(data: Vec<u8>) -> Self {
        parse(data.as_slice())
    }

    /// Parses a byte array to the BlockchainProtocol struct
    ///
    /// # Parameter
    ///
    /// - `data` - byte array that should be parsed
    ///
    /// # Return
    ///
    /// Parsed result of the byte array
    ///
    /// TODO: example
    pub fn from_u8(data: &[u8]) -> Self {
        parse(data)
    }

    /// Parses a string reference to the BlockchainProtocol struct
    ///
    /// # Parameter
    ///
    /// - `data` - string reference that should be parsed
    ///
    /// # Return
    ///
    /// Parsed result of the string reference
    ///
    /// TODO: example
    pub fn from_str(data: &str) -> Self {
        BlockchainProtocol::from_vec(data.from_hex())
    }

    /// Parses a String to the BlockchainProtocol struct
    ///
    /// # Parameter
    ///
    /// - `data` - String that should be parsed
    ///
    /// # Return
    ///
    /// Parsed result of the String
    ///
    /// TODO: example
    pub fn from_string(data: String) -> Self {
        BlockchainProtocol::from_vec(data.as_str().from_hex())
    }

    /// Sets the event code
    ///
    /// # Default
    ///
    /// EventCodes::NotAValidEvent
    ///
    /// # Parameters
    ///
    /// - `data` - payload that should be send
    ///
    /// # Return
    ///
    /// Updated instance of the struct
    pub fn set_event_code(mut self, event_code: EventCodes) -> Self {
        self.event_code = event_code;
        self
    }

    /// Sets the data that should be send as payload
    ///
    /// # Default
    ///
    /// Empty String
    ///
    /// # Parameters
    ///
    /// - `data` - payload that should be send
    ///
    /// # Return
    ///
    /// Updated instance of the struct
    pub fn set_data(mut self, data: String) -> Self {
        self.data = data.clone();
        self.data_length = data.len().to_string().parse::<u16>().unwrap();
        self
    }

    /// Combines the struct to a vector of bytes
    pub fn build(self) -> Vec<u8> {
        let slice_u16: &[u16] = &*vec![self.id, self.ttl, self.data_length];
        let converted_slice: &[u8] = unsafe {
            slice::from_raw_parts(
                slice_u16.as_ptr() as *const u8,
                slice_u16.len() * mem::size_of::<u16>(),
            )
        };

        let mut result: Vec<u8> = Vec::new();
        result.push(as_number(self.event_code));
        result.push(self.status_code);
        result.push(converted_slice[0]);
        result.push(converted_slice[1]);
        result.push(converted_slice[2]);
        result.push(converted_slice[3]);
        result.push(converted_slice[4]);
        result.push(converted_slice[5]);
        result
    }
}
