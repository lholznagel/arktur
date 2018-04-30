use base64::decode;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey;

/// Configuration for the peer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// List of all peers to connect to
    pub peers: Vec<Peer>,
    /// Port the peer should listen on
    pub port: u16,
    /// Pirvate key
    #[serde(rename="secretKey")]
    secret_key: String
}

/// Peer configuration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Peer {
    /// Address of the peer. Example: 127.0.0.1:4500
    pub address: String,
    /// Public Key of the peer
    #[serde(rename="publicKey")]
    public_key: String,
}

impl Peer {
    /// Returns the public key of a peer
    #[allow(dead_code)]
    pub fn public_key(&self) -> PublicKey {
        let decoded = decode(&self.public_key).unwrap();
        PublicKey::from_slice(&decoded).unwrap()
    }
}