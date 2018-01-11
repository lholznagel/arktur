use payload::PayloadModel;
use payload::ByteBuilder;
use std::str;
use std::mem::transmute;

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
/// // | Timestamp (signed)                                                                            |
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

impl PayloadModel for FoundBlockPayload {
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
        let mut content = Vec::new();
        for i in 0..bytes[0][0] {
            content.extend(bytes[(9 + i) as usize].iter());
        }

        let mut index_byte: [u8; 8] = [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        for i in 0..8 {
            index_byte[i] = bytes[4][i];
        }        
        let index = unsafe {
            transmute::<[u8; 8], u64>(index_byte)
        };

        let mut timestamp_byte: [u8; 8] = [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        for i in 0..8 {
            timestamp_byte[i] = bytes[5][i];
        }        
        let timestamp = unsafe {
            transmute::<[u8; 8], i64>(timestamp_byte)
        };

        let mut nonce_byte: [u8; 8] = [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        for i in 0..8 {
            nonce_byte[i] = bytes[6][i];
        }        
        let nonce = unsafe {
            transmute::<[u8; 8], u64>(nonce_byte)
        };

        Self {
            index: index,
            timestamp: timestamp,
            nonce: nonce,
            prev: String::from(str::from_utf8(&bytes[7]).unwrap()),
            hash: String::from(str::from_utf8(&bytes[8]).unwrap()),
            content: String::from(str::from_utf8(&content).unwrap()),
        }
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        ByteBuilder::new()
            .add_u8(((self.content.clone().len() as u64 / 255) as u8) + 1)
            .add_u8(0)
            .add_u8(0)
            .add_u8(0)
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

    fn parse_payload(payload: &[u8]) -> Vec<Vec<u8>> {
        let mut index: u64 = 0;
        let mut complete = Vec::new();

        if !payload.is_empty() {
            loop {
                if index == payload.len() as u64 {
                    break;
                }

                let mut current = Vec::new();
                let current_length = payload[index as usize];

                for i in (index + 1)..(index + current_length as u64 + 1) {
                    current.push(payload[i as usize]);
                    index += 1;
                }

                index += 1;
                complete.push(current);
            }
        }

        complete
    }

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

        let found_block = found_block.as_bytes();
        let complete = parse_payload(&found_block);
        let parsed = FoundBlockPayload::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(content, parsed.content);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
        assert_eq!(hash, parsed.hash);
    }

    #[test]
    fn test_long_string_1() {
        let index = 1465;
        let timestamp = 5825525;
        let nonce = 41684984;
        let prev = String::from("sdghnregneiurngnwg48g4g4erg46e4hh");
        let hash = String::from("asdmhgoirmhoiremh54651greher4h545");
        let content = "a".repeat(500);

        let found_block = FoundBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            hash: hash.clone(),
            content: content.clone()
        };

        let found_block = found_block.as_bytes();
        assert_eq!(found_block[1], 2);

        let complete = parse_payload(&found_block);
        let parsed = FoundBlockPayload::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(content, parsed.content);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
        assert_eq!(hash, parsed.hash);
    }

    #[test]
    fn test_long_string_2() {
        let index = 1465;
        let timestamp = 5825525;
        let nonce = 41684984;
        let prev = String::from("sdghnregneiurngnwg48g4g4erg46e4hh");
        let hash = String::from("asdmhgoirmhoiremh54651greher4h545");
        let content = "b".repeat(1000);

        let found_block = FoundBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            hash: hash.clone(),
            content: content.clone()
        };

        let found_block = found_block.as_bytes();
        assert_eq!(found_block[1], 4);

        let complete = parse_payload(&found_block);
        let parsed = FoundBlockPayload::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(content, parsed.content);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
        assert_eq!(hash, parsed.hash);
    }

    quickcheck! {
        fn test_quickcheck(index: u64, content: String, timestamp: i64, nonce: u64, prev: String, hash: String) -> bool {
            let index = index;
            let content = content;
            let timestamp = timestamp;
            let nonce = nonce;
            let prev = prev;
            let hash = hash;

            let found_block = FoundBlockPayload {
                index: index.clone(),
                timestamp: timestamp.clone(),
                nonce: nonce.clone(),
                prev: prev.clone(),
                hash: hash.clone(),
                content: content.clone()
            };

            let found_block = found_block.as_bytes();

            let complete = parse_payload(&found_block);
            let parsed = FoundBlockPayload::parse(complete);

            assert_eq!(index, parsed.index);
            assert_eq!(content, parsed.content);
            assert_eq!(timestamp, parsed.timestamp);
            assert_eq!(nonce, parsed.nonce);
            assert_eq!(prev, parsed.prev);
            assert_eq!(hash, parsed.hash);

            true
        }
    }
}