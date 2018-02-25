use payload::{Parser, Payload, PayloadBuilder};

use time::get_time;

/// Model for the event `NewBlock`
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Total Content         | Empty                                                                 |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Index (unsigned)                                                                              |
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Timestamp (unsigned)                                                                          |
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Prev                                                                                          |
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Signkey                                                                                       |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // |                                                                                               |
/// // //                                                                                             //
/// // // Content []                                                                                  //
/// // //                                                                                             //
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct NewBlockPayload {
    /// Index of the block
    pub index: u64,
    /// Timestamp the block was created
    pub timestamp: i64,
    /// Hash of the previous block
    pub prev: String,
    /// Sign key, the first values must match this string
    pub sign_key: String,
    /// Content of the block
    pub content: String
}

impl NewBlockPayload {
    /// Creates a new block
    pub fn block(index: u64, prev: String, content: String) -> Self {
        Self {
            index: index,
            timestamp: get_time().sec,
            prev: prev,
            sign_key: "0".repeat(4),
            content: content
        }
    }
}

impl Payload for NewBlockPayload {
    fn new() -> Self {
        Self {
            index: 0,
            timestamp: get_time().sec,
            prev: String::from(""),
            sign_key: "0".repeat(4),
            content: String::from("")
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            let content = Parser::string_overflow(&bytes[8..]);

            Self {
                index: Parser::u8_to_u64(bytes[4].as_slice()),
                timestamp: Parser::u8_to_u64(bytes[5].as_slice()) as i64,
                prev: Parser::u8_to_string(&bytes[6]),
                sign_key: Parser::u8_to_string(&bytes[7]),
                content: Parser::u8_to_string(&content)
            }
        } else {
            Self::new()
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new()
            .add_u8(((self.content.clone().len() as u64 / 255) as u8) + 1)
            .add_u8(0) // empty
            .add_u8(0) // empty
            .add_u8(0) // empty
            .add_u64(self.index)
            .add_u64(self.timestamp as u64) // will always be positiv
            .add_string(self.prev)
            .add_string(self.sign_key)
            .add_string_overflow(self.content)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payload::Parser;

    #[test]
    fn test_building_and_parsing() {
        let index = 4816;
        let timestamp = 54658;
        let prev = String::from("ngiurengoiurehgbiuergneoigjoierhg");
        let sign_key = String::from("0000");
        let content = String::from("Some string");

        let new_block = NewBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            sign_key: sign_key.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let new_block = new_block.to_bytes();
        let complete = Parser::parse_payload(&new_block);
        let parsed = NewBlockPayload::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(sign_key, parsed.sign_key);
        assert_eq!(prev, parsed.prev);
        assert_eq!(content, parsed.content);
    }

    #[test]
    fn test_long_string_1() {
        let index = 64587;
        let timestamp = 87451651;
        let prev = String::from("sdfsdgehherheherhefwt4wtttertertg");
        let sign_key = String::from("0000");
        let content = "a".repeat(500);

        let new_block = NewBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            sign_key: sign_key.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let new_block = new_block.to_bytes();
        assert_eq!(new_block[1], 2);

        let complete = Parser::parse_payload(&new_block);
        let parsed = NewBlockPayload::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(content, parsed.content);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(sign_key, parsed.sign_key);
        assert_eq!(prev, parsed.prev);
    }

    #[test]
    fn test_long_string_2() {
        let index = 521;
        let timestamp = 263514;
        let prev = String::from("gwegerhgerhgef2h6h4zh5j654mztkjh5");
        let sign_key = String::from("0000");
        let content = "b".repeat(1000);

        let new_block = NewBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            sign_key: sign_key.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let new_block = new_block.to_bytes();
        assert_eq!(new_block[1], 4);

        let complete = Parser::parse_payload(&new_block);
        let parsed = NewBlockPayload::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(content, parsed.content);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(sign_key, parsed.sign_key);
        assert_eq!(prev, parsed.prev);
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn test_quickcheck(index: u64, timestamp: i64, prev: String, sign_key: String, content: String) -> bool {
            let index = index;
            let timestamp = timestamp;
            let sign_key = sign_key;
            let prev = prev;
            let content = content;

            let new_block = NewBlockPayload {
                index: index.clone(),
                timestamp: timestamp.clone(),
                sign_key: sign_key.clone(),
                prev: prev.clone(),
                content: content.clone()
            };

            let new_block = new_block.to_bytes();

            let complete = Parser::parse_payload(&new_block);
            let parsed = NewBlockPayload::parse(complete);

            assert_eq!(index, parsed.index);
            assert_eq!(content, parsed.content);
            assert_eq!(timestamp, parsed.timestamp);
            assert_eq!(sign_key, parsed.sign_key);
            assert_eq!(prev, parsed.prev);
            true
        }
    }
}