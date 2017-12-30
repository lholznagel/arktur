use payload::PayloadModel;

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

    fn parse(_bytes: Vec<&[u8]>) -> Self {
        Self {
            index: 0,
            hash: String::from("")
        }
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        vec![0]
    }
}