use payload::{Payload, PayloadBuilder};

use std::str;

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
    /// Address of another peer
    pub addr: String,
}

impl Payload for RegisterAckPayload {
    fn new() -> Self {
        Self { addr: String::from("") }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            Self {
                addr: String::from(str::from_utf8(&bytes[0]).unwrap())
            }
        } else {
            Self::new()
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new()
            .add_string(self.addr)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payload::Parser;

    #[test]
    fn test_building_and_parsing() {
        let addr = String::from("172.0.0.1");

        let register_ack = RegisterAckPayload {
            addr: addr.clone()
        };

        let register_ack = register_ack.to_bytes();
        let complete = Parser::parse_payload(&register_ack);
        let parsed = RegisterAckPayload::parse(complete);

        assert_eq!(addr, parsed.addr);
    }

    quickcheck! {
        fn test_quickcheck(addr: String) -> bool {
            let addr = addr;

            let register_ack = RegisterAckPayload {
                addr: addr.clone()
            };

            let register_ack = register_ack.to_bytes();

            let complete = Parser::parse_payload(&register_ack);
            let parsed = RegisterAckPayload::parse(complete);

            assert_eq!(addr, parsed.addr);
            true
        }
    }
}