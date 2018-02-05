use payload::{Parser, Payload, PayloadBuilder};

/// Model for the event `FoundBlock`
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // | Unique key                                                                                    |
/// // |                                                                                               |
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
    /// contains a unique key so that the peers
    /// can check if they already know the content
    pub unique_key: String,
    /// Content for the next block
    pub content: String
}

impl Payload for DataForBlockPayload {
    fn new() -> Self {
        Self {
            unique_key: String::from(""),
            content: String::from("")
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            let content = Parser::string_overflow(&bytes[1..]);

            Self {
                unique_key: Parser::u8_to_string(&bytes[0]),
                content: Parser::u8_to_string(&content)
            }
        } else {
            Self::new()
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new()
            .add_string(self.unique_key)
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
        let unique_key = String::from("asfdkgrf");
        let content = String::from("ngiurengoiurehgbiuergneoigjoierhg");

        let data = DataForBlockPayload {
            unique_key: unique_key.clone(),
            content: content.clone()
        };

        let complete = Parser::parse_payload(&data.to_bytes());
        let parsed = DataForBlockPayload::parse(complete);

        assert_eq!(unique_key, parsed.unique_key);
        assert_eq!(content, parsed.content);
    }

    quickcheck! {
        fn test_quickcheck(unique_key: String, content: String) -> bool {
            let unique_key = unique_key;
            let content = content;

            let data = DataForBlockPayload {
                unique_key: unique_key.clone(),
                content: content.clone()
            };

            let complete = Parser::parse_payload(&data.to_bytes());
            let parsed = DataForBlockPayload::parse(complete);

            assert_eq!(unique_key, parsed.unique_key);
            assert_eq!(content, parsed.content);
            true
        }
    }
}