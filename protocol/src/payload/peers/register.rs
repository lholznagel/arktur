use payload::{Payload, Builder};
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey;
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Register {
    /// public key of the registering peer
    pub pub_key: PublicKey,
}

impl Register {
    /// Sets the peers that should be send
    pub fn set_pub_key(mut self, pub_key: PublicKey) -> Self {
        self.pub_key = pub_key;
        self
    }
}

impl Payload for Register {
    fn new() -> Self {
        Self { pub_key: PublicKey::from_slice(&[0; 32]).unwrap() }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Result<Self, ParseErrors> {
        if !bytes.is_empty() {
            let mut pub_key = [0; 32];
            for (i, val) in bytes[0].iter().enumerate() {
                pub_key[i] = *val;
            }

            Ok(Self {
                pub_key: PublicKey::from_slice(&pub_key).unwrap()
            })
        } else {
            Ok(Self::new())
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        Builder::new()
            .add_pub_key(self.pub_key.0)
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
        let (pub_key, _) = box_::gen_keypair();
        let register = Register {
            pub_key
        };

        let register = register.to_bytes();
        let complete = parser::parse_payload(&register);
        let parsed = Register::parse(complete).unwrap();

        assert_eq!(pub_key, parsed.pub_key);
    }
}