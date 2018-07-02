use failure::Error;
use payloads::Payload;
use protocol_builder_parser::{Builder, Parser};
use time;

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
/// // |                                                                                               |
/// // //                                                                                             //
/// // // Content []                                                                                  //
/// // //                                                                                             //
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct CalcBlockPayload {
    /// Index of the block
    pub index: u64,
    /// Timestamp the block was created
    pub timestamp: i32,
    /// Hash of the previous block
    pub prev: String,
    /// Content of the block
    pub content: String
}

impl CalcBlockPayload {
    /// Creates a new block
    pub fn block(index: u64, prev: String, content: String) -> Self {
        Self {
            index: index,
            timestamp: time::now_utc().tm_nsec / 1000,
            prev: prev,
            content: content
        }
    }
}

impl Payload for CalcBlockPayload {
    fn new() -> Self {
        Self {
            index: 0,
            timestamp: time::now_utc().tm_nsec / 1000,
            prev: String::new(),
            content: String::new()
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Result<Self, Error> {
        if !bytes.is_empty() {
            println!("{:?}", bytes);

            let content = Parser::combine(&bytes[7..]);

            let index = &Parser::vec_to_u8_8(bytes[4].clone())?;
            let timestamp = &Parser::vec_to_u8_4(bytes[5].clone())?;

            Ok(Self {
                index: Parser::to_u64(index),
                timestamp: Parser::to_u32(timestamp) as i32,
                prev: Parser::to_string(&bytes[6])?,
                content: Parser::to_string(&content)?
            })
        } else {
            Ok(Self::new())
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        Builder::new()
            .add_u8(((self.content.clone().len() as u64 / 255) as u8) + 1)
            .add_u8(0) // empty
            .add_u8(0) // empty
            .add_u8(0) // empty
            .add_u64(self.index)
            .add_u32(self.timestamp as u32)
            .add_string(self.prev)
            .add_string_overflow(self.content)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol_builder_parser::Parser;

    #[test]
    fn test_building_and_parsing() {
        let index = 4816;
        let timestamp = 54658;
        let prev = String::from("ngiurengoiurehgbiuergneoigjoierhg");
        let content = String::from("Some string");

        let new_block = CalcBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let new_block = new_block.to_bytes();
        let complete = Parser::parse_payload(&new_block);
        let parsed = CalcBlockPayload::parse(complete).unwrap();

        assert_eq!(index, parsed.index);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(prev, parsed.prev);
        assert_eq!(content, parsed.content);
    }

    #[test]
    fn test_long_string_1() {
        let index = 64587;
        let timestamp = 87451651;
        let prev = String::from("sdfsdgehherheherhefwt4wtttertertg");
        let content = "a".repeat(500);

        let new_block = CalcBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let new_block = new_block.to_bytes();
        assert_eq!(new_block[1], 2);

        let complete = Parser::parse_payload(&new_block);
        let parsed = CalcBlockPayload::parse(complete).unwrap();

        assert_eq!(index, parsed.index);
        assert_eq!(content, parsed.content);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(prev, parsed.prev);
    }

    #[test]
    fn test_long_string_2() {
        let index = 521;
        let timestamp = 263514;
        let prev = String::from("gwegerhgerhgef2h6h4zh5j654mztkjh5");
        let content = "b".repeat(1000);

        let new_block = CalcBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let new_block = new_block.to_bytes();
        assert_eq!(new_block[1], 4);

        let complete = Parser::parse_payload(&new_block);
        let parsed = CalcBlockPayload::parse(complete).unwrap();

        assert_eq!(index, parsed.index);
        assert_eq!(content, parsed.content);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(prev, parsed.prev);
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn test_quickcheck(index: u64, timestamp: i32, prev: String, content: String) -> bool {
            let index = index;
            let timestamp = timestamp;
            let prev = prev;
            let content = content;

            let new_block = CalcBlockPayload {
                index: index.clone(),
                timestamp: timestamp.clone(),
                prev: prev.clone(),
                content: content.clone()
            };

            let new_block = new_block.to_bytes();

            let complete = Parser::parse_payload(&new_block);
            let parsed = CalcBlockPayload::parse(complete).unwrap();

            assert_eq!(index, parsed.index);
            assert_eq!(content, parsed.content);
            assert_eq!(timestamp, parsed.timestamp);
            assert_eq!(prev, parsed.prev);
            true
        }
    }
}