use payload::{Parser, Payload, PayloadBuilder};

/// Model for the event `FoundBlock`
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
/// // |                                                                                               |
/// // //                                                                                             //
/// // // Content []                                                                                  //
/// // //                                                                                             //
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ValidateHash {
    /// Index of the block
    pub index: u64,
    /// Timestamp the block was created
    pub timestamp: i64,
    /// Nonce of the block
    pub nonce: u64,
    /// Hash of the previous block
    pub prev: String,
    /// Content of the block
    pub content: String,
}

impl Payload for ValidateHash {
    fn new() -> Self {
        Self {
            index: 0,
            timestamp: 0,
            nonce: 0,
            prev: String::from(""),
            content: String::from(""),
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            let content = Parser::string_overflow(&bytes[8..]);

            Self {
                index: Parser::u8_to_u64(bytes[4].as_slice()),
                timestamp: Parser::u8_to_u64(bytes[5].as_slice()) as i64,
                nonce: Parser::u8_to_u64(bytes[6].as_slice()),
                prev: Parser::u8_to_string(&bytes[7]),
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
            .add_u64(self.nonce)
            .add_string(self.prev)
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
        let index = 48;
        let timestamp = 69489;
        let nonce = 948;
        let prev = String::from("ngiurengoiurehgbiuergneoigjoierhg");
        let content = String::from("Some string");

        let validate_hash = ValidateHash {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let validate_hash = validate_hash.to_bytes();
        let complete = Parser::parse_payload(&validate_hash);
        let parsed = ValidateHash::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
        assert_eq!(content, parsed.content);
    }

    #[test]
    fn test_long_string_1() {
        let index = 64587;
        let timestamp = 87451651;
        let nonce = 948;
        let prev = String::from("sdfsdgehherheherhefwt4wtttertertg");
        let content = "a".repeat(500);

        let validate_hash = ValidateHash {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let validate_hash = validate_hash.to_bytes();
        assert_eq!(validate_hash[1], 2);

        let complete = Parser::parse_payload(&validate_hash);
        let parsed = ValidateHash::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(content, parsed.content);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
    }

    #[test]
    fn test_long_string_2() {
        let index = 521;
        let timestamp = 263514;
        let nonce = 948;
        let prev = String::from("gwegerhgerhgef2h6h4zh5j654mztkjh5");
        let content = "b".repeat(1000);

        let validate_hash = ValidateHash {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let validate_hash = validate_hash.to_bytes();
        assert_eq!(validate_hash[1], 4);

        let complete = Parser::parse_payload(&validate_hash);
        let parsed = ValidateHash::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(content, parsed.content);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
    }

    quickcheck! {
        fn test_quickcheck(index: u64, timestamp: i64, nonce: u64, prev: String, content: String) -> bool {
            let index = index;
            let nonce = nonce;
            let timestamp = timestamp;
            let prev = prev;
            let content = content;

            let validate_hash = ValidateHash {
                index: index.clone(),
                timestamp: timestamp.clone(),
                nonce: nonce.clone(),
                prev: prev.clone(),
                content: content.clone()
            };

            let validate_hash = validate_hash.to_bytes();

            let complete = Parser::parse_payload(&validate_hash);
            let parsed = ValidateHash::parse(complete);

            assert_eq!(index, parsed.index);
            assert_eq!(content, parsed.content);
            assert_eq!(timestamp, parsed.timestamp);
            assert_eq!(nonce, parsed.nonce);
            assert_eq!(prev, parsed.prev);
            true
        }
    }
}