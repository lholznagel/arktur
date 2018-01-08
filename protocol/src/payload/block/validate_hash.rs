use payload::PayloadModel;
use std::str;

/// Model for the event `FoundBlock`
#[derive(Debug, PartialEq)]
pub struct ValidateHash {
    /// Index of the block
    pub index: u64,
    /// Content of the block
    pub content: String,
    /// Timestamp the block was created
    pub timestamp: i64,
    /// Hash of the previous block
    pub prev: String,
    /// Nonce of the block
    pub nonce: u64,
}

impl PayloadModel for ValidateHash {
    fn new() -> Self {
        Self {
            index: 0,
            content: String::from(""),
            timestamp: 0,
            prev: String::from(""),
            nonce: 0
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        Self {
            index: String::from(str::from_utf8(&bytes[0]).unwrap()).parse::<u64>().unwrap(),
            content: String::from(str::from_utf8(&bytes[1]).unwrap()),
            timestamp: String::from(str::from_utf8(&bytes[2]).unwrap()).parse::<i64>().unwrap(),
            prev: String::from(str::from_utf8(&bytes[3]).unwrap()),
            nonce: String::from(str::from_utf8(&bytes[4]).unwrap()).parse::<u64>().unwrap(),
        }
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        result.push(126);
        for i in self.index.to_string().into_bytes() {
            result.push(i);
        }
        result.push(126);

        result.push(126);
        for i in self.content.into_bytes() {
            result.push(i);
        }
        result.push(126);

        result.push(126);
        for i in self.timestamp.to_string().into_bytes() {
            result.push(i);
        }
        result.push(126);

        result.push(126);
        for i in self.prev.into_bytes() {
            result.push(i);
        }
        result.push(126);

        result.push(126);
        for i in self.nonce.to_string().into_bytes() {
            result.push(i);
        }
        result.push(126);
        result
    }
}