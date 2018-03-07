use payload::{Parser, Payload, PayloadBuilder};

/// Model for the event `RegisterAck`
///
/// ```
/// //  00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// // |                                                                                               |
/// // // Address                                                                                       |
/// // |                                                                                               |
/// // +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct RegisterAckPayload {
    /// Addresses of all peers
    pub addresses: Vec<String>,
}

impl RegisterAckPayload {
    /// Sets the peers that should be send
    pub fn set_peers(mut self, peers: Vec<String>) -> Self {
        self.addresses = peers;
        self
    }
}

impl Payload for RegisterAckPayload {
    fn new() -> Self {
        Self { addresses: Vec::new() }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            let content = Parser::string_overflow(&bytes[0..]);
            let stuff = String::from(Parser::u8_to_string(&content));
            let mut result: Vec<String> = Vec::new();

            for address in stuff.split(", ").collect::<Vec<&str>>() {
                result.push(String::from(address));
            }

            Self {
                addresses: result
            }
        } else {
            Self::new()
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new()
            .add_string_overflow(self.addresses.join(", "))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payload::Parser;

    #[test]
    fn test_building_and_parsing() {
        let addresses = vec![String::from("172.0.0.1")];

        let register_ack = RegisterAckPayload {
            addresses: addresses.clone()
        };

        let register_ack = register_ack.to_bytes();
        let complete = Parser::parse_payload(&register_ack);
        let parsed = RegisterAckPayload::parse(complete);

        assert_eq!(addresses, parsed.addresses);
    }
}