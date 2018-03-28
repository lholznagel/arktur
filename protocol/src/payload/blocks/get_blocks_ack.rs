use payload::{Payload, Builder};
use errors::ParseErrors;

/// Model for the event `RegisterAck`
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // |                                                                                               |
/// // // Blocks                                                                                       |
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct GetBlocksAck {
    /// blocks of all blocks
    pub blocks: Vec<String>,
}

impl Payload for GetBlocksAck {
    fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Result<Self, ParseErrors> {
        if !bytes.is_empty() {
            let mut blocks = Vec::new();

            for byte in bytes {
                if !byte.is_empty() {
                    blocks.push(String::from_utf8(byte).unwrap());
                }
            }

            Ok(Self {
                blocks
            })
        } else {
            Ok(Self::new())
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        Builder::new()
            .add_string_vector(self.blocks)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payload::parser;

    #[test]
    fn test_building_and_parsing_empty() {
        let block_ack = GetBlocksAck {
            blocks: Vec::new()
        };

        let block_ack = block_ack.to_bytes();
        let complete = parser::parse_payload(&block_ack);
        let parsed = GetBlocksAck::parse(complete).unwrap();

        assert_eq!(Vec::<String>::new(), parsed.blocks);
    }

    #[test]
    fn test_building_and_parsing() {
        let blocks = vec![String::from("172.0.0.1"), String::from("172.0.0.2")];

        let block_ack = GetBlocksAck {
            blocks: blocks.clone()
        };

        let block_ack = block_ack.to_bytes();
        let complete = parser::parse_payload(&block_ack);
        let parsed = GetBlocksAck::parse(complete).unwrap();

        assert_eq!(blocks, parsed.blocks);
    }
}