use payload::PayloadModel;

/// Model for the event `PossibleBlock`
#[derive(Debug, PartialEq)]
pub struct PossibleBlockPayload {
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
    /// Generated hash that respects all values
    pub hash: String,
}

impl PayloadModel for PossibleBlockPayload {
    fn new() -> Self {
        PossibleBlockPayload {
            index: 0,
            content: String::from(""),
            timestamp: 0,
            prev: String::from(""),
            nonce: 0,
            hash: String::from(""),
        }
    }

    fn parse(_bytes: Vec<&[u8]>) -> Self {
        PossibleBlockPayload {
            index: 0,
            content: String::from(""),
            timestamp: 0,
            prev: String::from(""),
            nonce: 0,
            hash: String::from(""),
        }
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        vec![0]
    }
}