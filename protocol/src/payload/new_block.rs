use payload::PayloadModel;

use std::str;
use time::get_time;

/// Model for the event `NewBlock`
#[derive(Debug, PartialEq)]
pub struct NewBlockPayload {
    /// Index of the block
    pub index: u64,
    /// Content of the block
    pub content: String,
    /// Timestamp the block was created
    pub timestamp: i64,
    /// Hash of the previous block
    pub prev: String,
}

impl NewBlockPayload {
    /// Generates a genesis block
    pub fn genesis() -> Self {
        NewBlockPayload {
            index: 0,
            content: String::from(""),
            timestamp: get_time().sec,
            prev: String::from("0".repeat(64))
        }
    }

    /// Sets the index of the block
    pub fn set_index(mut self, index: u64) -> Self {
        self.index = index;
        self
    }

    /// Sets the content of the block
    pub fn set_content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    /// Sets the previous hash
    pub fn set_prev(mut self, prev: String) -> Self {
        self.prev = prev;
        self
    }
}

impl PayloadModel for NewBlockPayload {
    fn new() -> Self {
        NewBlockPayload {
            index: 0,
            content: String::from(""),
            timestamp: get_time().sec,
            prev: String::from("")
        }
    }

    fn parse(bytes: Vec<&[u8]>) -> Self {
        if bytes.len() > 0 {
            NewBlockPayload {
                index: String::from(str::from_utf8(bytes[0]).unwrap()).parse::<u64>().unwrap(),
                content: String::from(str::from_utf8(bytes[1]).unwrap()),
                timestamp: String::from(str::from_utf8(bytes[2]).unwrap()).parse::<i64>().unwrap(),
                prev: String::from(str::from_utf8(bytes[3]).unwrap())
            }
        } else {
            NewBlockPayload::new()
        }
    }

    /// TODO: implement this
    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        // 126 as char equals ~
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

        result
    }
}