use payload::{parser, Payload, PayloadBuilder};

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
pub struct GetPeersAckPayload {
    /// peers of all peers
    pub peers: Vec<String>,
}

impl GetPeersAckPayload {
    /// Sets the peers that should be send
    pub fn set_peers(mut self, peers: Vec<String>) -> Self {
        self.peers = peers;
        self
    }
}

impl Payload for GetPeersAckPayload {
    fn new() -> Self {
        Self { peers: Vec::new() }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if !bytes.is_empty() {
            let content = parser::string_overflow(&bytes[0..]);
            let peers = String::from(parser::u8_to_string(&content));
            let mut result: Vec<String> = Vec::new();

            for peer in peers.split(", ").collect::<Vec<&str>>() {
                result.push(String::from(peer));
            }

            Self {
                peers: result
            }
        } else {
            Self::new()
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new()
            .add_string_overflow(self.peers.join(", "))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payload::parser;

    #[test]
    fn test_building_and_parsing() {
        let peers = vec![String::from("172.0.0.1"), String::from("172.0.0.2")];

        let register_ack = GetPeersAckPayload {
            peers: peers.clone()
        };

        let register_ack = register_ack.to_bytes();
        let complete = parser::parse_payload(&register_ack);
        let parsed = GetPeersAckPayload::parse(complete);

        assert_eq!(peers, parsed.peers);
    }
}