use failure::Error;
use payload::{parser, Payload, Builder};

/// Model for the event `RegisterAck`
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Block                                         |                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct GetBlock {
    /// requested block
    pub block: String,
}

impl Payload for GetBlock {
    fn new() -> Self {
        Self { block: String::new() }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Result<Self, Error> {
        if !bytes.is_empty() {
            Ok(Self {
                block: parser::u8_to_string(&bytes[0])?,
            })
        } else {
            Ok(Self::new())
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        Builder::new()
            .add_string(self.block)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payload::parser;

    #[test]
    fn test_building_and_parsing() {
        let block = String::from("ABCDEFGHIJKLMNOP");

        let block_ack = GetBlock {
            block: block.clone()
        };

        let block_ack = block_ack.to_bytes();
        let complete = parser::parse_payload(&block_ack);
        let parsed = GetBlock::parse(complete).unwrap();

        assert_eq!(block, parsed.block);
    }
}