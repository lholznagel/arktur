use failure::Error;
use payloads::Payload;
use protocol_builder_parser::{Builder, Parser};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::thread_rng;

/// Model for the event `NewBlockContentPayload`
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
pub struct NewBlockContentPayload {
    /// contains a unique key so that the peers
    /// can check if they already know the content
    pub unique_key: String,
    /// Content for the next block
    pub content: String
}

impl Payload for NewBlockContentPayload {
    fn new() -> Self {
        let unique_key = thread_rng().sample_iter(&Alphanumeric).take(16).collect::<String>();

        Self {
            unique_key,
            content: String::new()
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Result<Self, Error> {
        if !bytes.is_empty() {
            let content = Parser::combine(&bytes[1..]);

            Ok(Self {
                unique_key: Parser::to_string(&bytes[0])?,
                content: Parser::to_string(&content)?
            })
        } else {
            Ok(Self::new())
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        Builder::new()
            .add_string_capacity(16, self.unique_key)
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
        let unique_key = String::from(thread_rng().sample_iter(&Alphanumeric).take(16).collect::<String>());
        let content = String::from("SomeCoolContent");

        let data = NewBlockContentPayload {
            unique_key: unique_key.clone(),
            content: content.clone()
        };

        let complete = Parser::parse_payload(&data.to_bytes());
        let parsed = NewBlockContentPayload::parse(complete).unwrap();

        assert_eq!(unique_key, parsed.unique_key);
        assert_eq!(content, parsed.content);
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn test_quickcheck(content: String) -> bool {
            let content = content;

            let mut data = NewBlockContentPayload::new();
            data.content = content.clone();

            let complete = Parser::parse_payload(&data.to_bytes());
            let parsed = NewBlockContentPayload::parse(complete).unwrap();

            assert_eq!(content, parsed.content);
            true
        }
    }
}