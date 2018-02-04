use payload::{Parser, Payload, PayloadBuilder};

/// Model for the event `FoundBlock`
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // |                                                                                               |
/// // //                                                                                             //
/// // // Content                                                                                     //
/// // //                                                                                             //
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct DataForBlockPayload {
    /// Index of the block
    pub content: String
}

impl Payload for DataForBlockPayload {
    fn new() -> Self {
        Self {
            content: String::from("")
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            let content = Parser::string_overflow(&bytes[0..]);

            Self {
                content: Parser::u8_to_string(&content)
            }
        } else {
            Self::new()
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new()
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
        let content = String::from("ngiurengoiurehgbiuergneoigjoierhg");

        let data = DataForBlockPayload {
            content: content.clone()
        };

        let complete = Parser::parse_payload(&data.to_bytes());
        let parsed = DataForBlockPayload::parse(complete);

        assert_eq!(content, parsed.content);
    }

    quickcheck! {
        fn test_quickcheck(content: String) -> bool {
            let content = content;

            let data = DataForBlockPayload {
                content: content.clone()
            };

            let complete = Parser::parse_payload(&data.to_bytes());
            let parsed = DataForBlockPayload::parse(complete);

            assert_eq!(content, parsed.content);
            true
        }
    }
}