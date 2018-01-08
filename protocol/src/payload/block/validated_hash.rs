use payload::PayloadModel;
use std::str;

/// Model for the event `FoundBlock`
#[derive(Debug, PartialEq)]
pub struct ValidatedHash {
    /// Index of the block
    pub index: u64,
    /// Hash of the block
    pub hash: String
}

impl PayloadModel for ValidatedHash {
    fn new() -> Self {
        Self {
            index: 0,
            hash: String::from("")
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        Self {
            index: String::from(str::from_utf8(&bytes[0]).unwrap()).parse::<u64>().unwrap(),
            hash: String::from(str::from_utf8(&bytes[1]).unwrap())
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
        for i in self.hash.into_bytes() {
            result.push(i);
        }
        result.push(126);
        result
    }
}