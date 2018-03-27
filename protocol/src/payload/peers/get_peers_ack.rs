use payload::{parser, Payload, Builder};
use errors::ParseErrors;

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
pub struct GetPeersAck {
    /// peers of all peers
    pub peers: Vec<String>,
}

impl GetPeersAck {
    /// Sets the peers that should be send
    pub fn set_peers(mut self, peers: Vec<String>) -> Self {
        self.peers = peers;
        self
    }
}

impl Payload for GetPeersAck {
    fn new() -> Self {
        Self { peers: Vec::new() }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Result<Self, ParseErrors> {
        if !bytes.is_empty() {
            let mut peers = Vec::new();

            for byte in bytes {
                let parsed = parser::u8_to_string_vec(&byte);
                peers.extend(parsed);
            }

            Ok(Self {
                peers
            })
        } else {
            Ok(Self::new())
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        Builder::new()
            .add_string_vector(self.peers)
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

        let register_ack = GetPeersAck {
            peers: peers.clone()
        };

        let register_ack = register_ack.to_bytes();
        let complete = parser::u8_to_string_vec(&register_ack);

        assert_eq!(peers, complete);
    }
}