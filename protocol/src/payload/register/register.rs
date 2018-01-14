use payload::{Parser, Payload, PayloadBuilder};

/// Model for the event `Register`
#[derive(Clone, Debug, PartialEq)]
pub struct RegisterPayload {
    /// Name of the peer
    pub name: String,
}

impl Payload for RegisterPayload {
    fn new() -> Self {
        Self { name: String::from("") }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            Self {
                name: Parser::u8_to_string(&bytes[0])
            }
        } else {
            Self::new()
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new()
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

        let register = register.to_bytes();
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

            let register = register.to_bytes();

            let complete = Parser::parse_payload(&register);
            let parsed = RegisterPayload::parse(complete);

            assert_eq!(name, parsed.name);
            true
        }
    }
}