use payload::{parser, Payload, PayloadBuilder};

/// Model for the event `FoundBlock`
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Index (unsigned)                                                                              |
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Hash                                                                                          |
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct HashValAck {
    /// Index of the block
    pub index: u64,
    /// Hash of the block
    pub hash: String
}

impl Payload for HashValAck {
    fn new() -> Self {
        Self {
            index: 0,
            hash: String::from("")
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            Self {
                index: parser::u8_to_u64(bytes[0].as_slice()),
                hash: parser::u8_to_string(&bytes[1])
            }
        } else {
            Self::new()
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new()
            .add_u64(self.index)
            .add_string(self.hash)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payload::parser;

    #[test]
    fn test_building_and_parsing() {
        let index = 65485;
        let hash = String::from("ngiurengoiurehgbiuergneoigjoierhg");

        let validated_hash = HashValAck {
            index: index.clone(),
            hash: hash.clone()
        };

        let validated_hash = validated_hash.to_bytes();
        let complete = parser::parse_payload(&validated_hash);
        let parsed = HashValAck::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(hash, parsed.hash);
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn test_quickcheck(index: u64, hash: String) -> bool {
            let index = index;
            let hash = hash;

            let validated_hash = HashValAck {
                index: index.clone(),
                hash: hash.clone()
            };

            let validated_hash = validated_hash.to_bytes();

            let complete = parser::parse_payload(&validated_hash);
            let parsed = HashValAck::parse(complete);

            assert_eq!(index, parsed.index);
            assert_eq!(hash, parsed.hash);
            true
        }
    }
}