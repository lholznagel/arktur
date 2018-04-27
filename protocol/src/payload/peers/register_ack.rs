use failure::Error;
use payload::{parser, Payload, Builder};
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey;

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
pub struct RegisterAck {
    /// public key of the peer
    pub public_key: Option<PublicKey>,
    /// peers of all peers
    pub peers: Vec<String>,
}

impl RegisterAck {
    /// Sets the peers that should be send
    pub fn set_peers(mut self, peers: Vec<String>) -> Self {
        self.peers = peers;
        self
    }

    /// Sets the public key
    pub fn set_public_key(mut self, public_key: &PublicKey) -> Self {
        self.public_key = Some(*public_key);
        self
    }
}

impl Payload for RegisterAck {
    fn new() -> Self {
        Self { 
            public_key: None,
            peers: Vec::new() 
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Result<Self, Error> {
        if !bytes.is_empty() {
            let public_key = PublicKey::from_slice(&bytes[0]);
            let mut peers = Vec::new();

            for byte in bytes[1..].iter() {
                if !byte.is_empty() {
                    peers.push(parser::u8_to_string(&byte)?);
                }
            }

            Ok(Self {
                public_key,
                peers
            })
        } else {
            Ok(Self::new())
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        Builder::new()
            .add_public_key(self.public_key.unwrap().0)
            .add_string_vector(self.peers)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nacl::Nacl;
    use payload::parser;

    #[test]
    fn test_building_and_parsing_empty() {
        let our_nacl = Nacl::new();

        let register_ack = RegisterAck {
            public_key: Some(our_nacl.get_public_key()),
            peers: Vec::new()
        };

        let register_ack = register_ack.to_bytes();
        let complete = parser::parse_payload(&register_ack);
        let parsed = RegisterAck::parse(complete).unwrap();

        assert_eq!(Vec::<String>::new(), parsed.peers);
    }

    #[test]
    fn test_building_and_parsing() {
        let our_nacl = Nacl::new();
        let peers = vec![String::from("172.0.0.1"), String::from("172.0.0.2")];

        let register_ack = RegisterAck {
            public_key: Some(our_nacl.get_public_key()),
            peers: peers.clone()
        };

        let register_ack = register_ack.to_bytes();
        let complete = parser::parse_payload(&register_ack);
        let parsed = RegisterAck::parse(complete).unwrap();

        assert_eq!(peers, parsed.peers);
    }
}