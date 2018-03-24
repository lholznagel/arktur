use payload::{parser, Payload, PayloadBuilder};
use protocol::ParseErrors;

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
pub struct GetBlockAck {
    /// filename
    pub filename: String,
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

impl Payload for GetBlockAck {
    fn new() -> Self {
        Self {
            filename: String::new(),
            index: 0,
            timestamp: 0,
            nonce: 0,
            prev: String::from(""),
            hash: String::from(""),
            content: String::from("")
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Result<Self, ParseErrors> {
        if !bytes.is_empty() {
            let content = parser::string_overflow(&bytes[8..]);
            let index = match parser::u8_to_u64(bytes[3].as_slice()) {
                Ok(val) => val,
                Err(_) => return Err(ParseErrors::NotEnoughBytes)
            };

            let timestamp = match parser::u8_to_u64(bytes[4].as_slice()) {
                Ok(val) => val as i64,
                Err(_) => return Err(ParseErrors::NotEnoughBytes)
            };

            let nonce = match parser::u8_to_u64(bytes[5].as_slice()) {
                Ok(val) => val,
                Err(_) => return Err(ParseErrors::NotEnoughBytes)
            };

            Ok(Self {
                filename: parser::u8_to_string(&bytes[1]),
                index,
                timestamp,
                nonce,
                prev: parser::u8_to_string(&bytes[6]),
                hash: parser::u8_to_string(&bytes[7]),
                content: parser::u8_to_string(&content),
            })
        } else {
            Ok(Self::new())
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new()
            .add_u8(((self.content.clone().len() as u64 / 255) as u8) + 1)
            .add_string(self.filename)
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
    use payload::parser;

    #[test]
    fn test_building_and_parsing() {
        let filename = String::from("SmallHash");
        let index = 1465;
        let timestamp = 5825525;
        let nonce = 41684984;
        let prev = String::from("asdsadasdasdagerjh5k78k");
        let hash = String::from("6j56j65j65jrtjrgqwfdsav");
        let content = String::from("Some string");

        let payload = GetBlockAck {
            filename: filename.clone(),
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            hash: hash.clone(),
            content: content.clone()
        };

        let payload = payload.to_bytes();
        let complete = parser::parse_payload(&payload);
        let parsed = GetBlockAck::parse(complete).unwrap();

        assert_eq!(filename, parsed.filename);
        assert_eq!(index, parsed.index);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
        assert_eq!(hash, parsed.hash);
        assert_eq!(content, parsed.content);
    }

    #[test]
    fn test_long_string_1() {
        let filename = String::from("SmallHash");
        let index = 1465;
        let timestamp = 5825525;
        let nonce = 41684984;
        let prev = String::from("grehzrtjztnjdfbvsnbtnrtednhrtd");
        let hash = String::from("nrtnrbwefvrmukjgfbdvshbjnrtnrh");
        let content = "a".repeat(500);

        let payload = GetBlockAck {
            filename: filename.clone(),
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            hash: hash.clone(),
            content: content.clone()
        };

        let payload = payload.to_bytes();
        assert_eq!(payload[1], 2);

        let complete = parser::parse_payload(&payload);
        let parsed = GetBlockAck::parse(complete).unwrap();

        assert_eq!(filename, parsed.filename);
        assert_eq!(index, parsed.index);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
        assert_eq!(hash, parsed.hash);
        assert_eq!(content, parsed.content);
    }

    #[test]
    fn test_long_string_2() {
        let filename = String::from("SmallHash");
        let index = 1465;
        let timestamp = 5825525;
        let nonce = 41684984;
        let prev = String::from("mtzusgtrjztjnhndvmbrebgrebreb");
        let hash = String::from("sgehnefindsivnhsfsdfiudnusdgn");
        let content = "b".repeat(1000);

        let payload = GetBlockAck {
            filename: filename.clone(),
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            hash: hash.clone(),
            content: content.clone()
        };

        let payload = payload.to_bytes();
        assert_eq!(payload[1], 4);

        let complete = parser::parse_payload(&payload);
        let parsed = GetBlockAck::parse(complete).unwrap();

        assert_eq!(filename, parsed.filename);
        assert_eq!(index, parsed.index);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
        assert_eq!(hash, parsed.hash);
        assert_eq!(content, parsed.content);
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn test_quickcheck(filename: String, index: u64, timestamp: i64, nonce: u64, hash: String, prev: String, content: String) -> bool {
            let filename = filename;
            let index = index;
            let timestamp = timestamp;
            let nonce = nonce;
            let prev = prev;
            let hash = hash;
            let content = content;

            let payload = GetBlockAck {
                filename: filename.clone(),
                index: index.clone(),
                timestamp: timestamp.clone(),
                nonce: nonce.clone(),
                prev: prev.clone(),
                hash: hash.clone(),
                content: content.clone()
            };

            let payload = payload.to_bytes();

            let complete = parser::parse_payload(&payload);
            let parsed = GetBlockAck::parse(complete).unwrap();

            assert_eq!(filename, parsed.filename);
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