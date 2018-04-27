use failure::Error;
use payload::{Payload, Builder};
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Register {
    /// public key of the registering peer
    pub public_key: PublicKey,
}

impl Register {
    /// Sets the peers that should be send
    pub fn set_public_key(mut self, public_key: PublicKey) -> Self {
        self.public_key = public_key;
        self
    }
}

impl Payload for Register {
    fn new() -> Self {
        Self { public_key: PublicKey::from_slice(&[0; 32]).unwrap() }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Result<Self, Error> {
        if !bytes.is_empty() {
            let mut public_key = [0; 32];
            for (i, val) in bytes[0].iter().enumerate() {
                public_key[i] = *val;
            }

            Ok(Self {
                public_key: PublicKey::from_slice(&public_key).unwrap()
            })
        } else {
            Ok(Self::new())
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        Builder::new()
            .add_public_key(self.public_key.0)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use payload::parser;
    use sodiumoxide::crypto::box_;

    #[test]
    fn test_building_and_parsing() {
        let (public_key, _) = box_::gen_keypair();
        let register = Register {
            public_key
        };

        let register = register.to_bytes();
        let complete = parser::parse_payload(&register);
        let parsed = Register::parse(complete).unwrap();

        assert_eq!(public_key, parsed.public_key);
    }
}