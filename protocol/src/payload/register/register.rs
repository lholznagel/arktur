use payload::PayloadModel;
use payload::ByteBuilder;

use std::str;

/// Model for the event `Register`
#[derive(Clone, Debug, PartialEq)]
pub struct RegisterPayload {
    /// Name of the peer
    pub name: String,
}

impl PayloadModel for RegisterPayload {
    fn new() -> Self {
        Self { name: String::from("") }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            Self {
                name: String::from(str::from_utf8(&bytes[0]).unwrap())
            }
        } else {
            Self::new()
        }
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        ByteBuilder::new()
            .add_string(self.name)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payload::Parser;

    #[test]
    fn test_building_and_parsing() {
        let name = String::from("172.0.0.1");

        let register = RegisterPayload {
            name: name.clone()
        };

        let register = register.as_bytes();
        let complete = Parser::parse_payload(&register);
        let parsed = RegisterPayload::parse(complete);

        assert_eq!(name, parsed.name);
    }

    quickcheck! {
        fn test_quickcheck(name: String) -> bool {
            let name = name;

            let register = RegisterPayload {
                name: name.clone()
            };

            let register = register.as_bytes();

            let complete = Parser::parse_payload(&register);
            let parsed = RegisterPayload::parse(complete);

            assert_eq!(name, parsed.name);
            true
        }
    }
}