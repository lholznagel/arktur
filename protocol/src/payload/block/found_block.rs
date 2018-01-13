use payload::{Parser, Payload, PayloadBuilder};

use std::str;

/// Struct of the FoundBlock payload
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
/// // | Nonce (unsigned)                                                                              |
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Prev                                                                                          |
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Hash                                                                                          |
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
pub struct FoundBlockPayload {
    /// Index of the block
    pub index: u64,
    /// Timestamp the block was created
    pub timestamp: i64,
    /// Nonce for this block
    pub nonce: u64,
    /// Hash of the previous block
    pub prev: String,
    /// Hash of this block
    pub hash: String,
    /// Content of the block
    pub content: String
}

impl Payload for FoundBlockPayload {
    fn new() -> Self {
        Self {
            index: 0,
            timestamp: 0,
            nonce: 0,
            prev: String::from(""),
            hash: String::from(""),
            content: String::from("")
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            let content = Parser::string_overflow(bytes[0][0], 9, bytes.clone());

            Self {
                index: Parser::u8_to_u64(bytes[4].as_slice()),
                timestamp: Parser::u8_to_u64(bytes[5].as_slice()) as i64,
                nonce: Parser::u8_to_u64(bytes[6].as_slice()),
                prev: String::from(str::from_utf8(&bytes[7]).unwrap()),
                hash: String::from(str::from_utf8(&bytes[8]).unwrap()),
                content: String::from(str::from_utf8(&content).unwrap()),
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
            .add_u64(self.nonce)
            .add_string(self.prev)
            .add_string(self.hash)
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
        let index = 1465;
        let timestamp = 5825525;
        let nonce = 41684984;
        let prev = String::from("sdghnregneiurngnwg48g4g4erg46e4hh");
        let hash = String::from("asdmhgoirmhoiremh54651greher4h545");
        let content = String::from("Some string");

        let found_block = FoundBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            hash: hash.clone(),
            content: content.clone()
        };

        let found_block = found_block.to_bytes();
        let complete = Parser::parse_payload(&found_block);
        let parsed = FoundBlockPayload::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
        assert_eq!(hash, parsed.hash);
        assert_eq!(content, parsed.content);
    }

    #[test]
    fn test_long_string_1() {
        let index = 1465;
        let timestamp = 5825525;
        let nonce = 41684984;
        let prev = String::from("hrjh4ej65kj56j56j56grtjthhgrjjrgjr");
        let hash = String::from("tjhtjrjfjhngpfiwshgwtw98tu345z48h4");
        let content = "a".repeat(500);

        let found_block = FoundBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            hash: hash.clone(),
            content: content.clone()
        };

        let found_block = found_block.to_bytes();
        assert_eq!(found_block[1], 2);

        let complete = Parser::parse_payload(&found_block);
        let parsed = FoundBlockPayload::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
        assert_eq!(hash, parsed.hash);
        assert_eq!(content, parsed.content);
    }

    #[test]
    fn test_long_string_2() {
        let index = 1465;
        let timestamp = 5825525;
        let nonce = 41684984;
        let prev = String::from("ergregergergegk04u9geogmgehe9hm39");
        let hash = String::from("fwgn2ogh238hg29gpfj9wpjppw0efklfh");
        let content = "b".repeat(1000);

        let found_block = FoundBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            hash: hash.clone(),
            content: content.clone()
        };

        let found_block = found_block.to_bytes();
        assert_eq!(found_block[1], 4);

        let complete = Parser::parse_payload(&found_block);
        let parsed = FoundBlockPayload::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
        assert_eq!(hash, parsed.hash);
        assert_eq!(content, parsed.content);
    }

    quickcheck! {
        fn test_quickcheck(index: u64, timestamp: i64, nonce: u64, hash: String, prev: String, content: String) -> bool {
            let index = index;
            let timestamp = timestamp;
            let nonce = nonce;
            let prev = prev;
            let hash = hash;
            let content = content;

            let found_block = FoundBlockPayload {
                index: index.clone(),
                timestamp: timestamp.clone(),
                nonce: nonce.clone(),
                prev: prev.clone(),
                hash: hash.clone(),
                content: content.clone()
            };

            let found_block = found_block.to_bytes();

            let complete = Parser::parse_payload(&found_block);
            let parsed = FoundBlockPayload::parse(complete);

            assert_eq!(index, parsed.index);
            assert_eq!(timestamp, parsed.timestamp);
            assert_eq!(nonce, parsed.nonce);
            assert_eq!(prev, parsed.prev);
            assert_eq!(hash, parsed.hash);
            assert_eq!(content, parsed.content);
            true
        }
    }
}