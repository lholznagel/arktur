use base64::decode;
use carina_peer;
use carina_peer::config::{Config, Peer};
use clap::ArgMatches;
use serde_yaml;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{PublicKey, SecretKey};
use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    /// List of all peers to connect to
    pub peers: Vec<PeerConfig>,
    /// Port the peer should listen on
    pub port: u16,
    /// Storage for the blocks
    pub storage: String,
    /// Pirvate key
    #[serde(rename="secretKey")]
    secret_key: String
}

/// Peer configuration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeerConfig {
    /// Address of the peer. Example: 127.0.0.1:4500
    pub address: String,
    /// Public Key of the peer
    #[serde(rename="publicKey")]
    public_key: String,
}

impl Configuration {
    pub fn get_config(self) -> Config {
        let secret_decoded = decode(&self.secret_key).unwrap();
        let secret = SecretKey::from_slice(&secret_decoded).unwrap();
        let mut peers = Vec::new();

        for peer in self.peers {
            peers.push(peer.get_config());
        }

        Config::new(self.port, peers, self.storage, secret)
    }
}

impl PeerConfig {
    pub fn get_config(self) -> Peer {
        let public_decoded = decode(&self.public_key).unwrap();
        let public = PublicKey::from_slice(&public_decoded).unwrap();

        Peer {
            address: self.address,
            public_key: public,
        }
    }
}

pub fn execute(args: &ArgMatches) {
    let mut file = File::open(args.value_of("CONFIG").unwrap().to_string()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config: Configuration = serde_yaml::from_str(&content).unwrap();

    carina_peer::init(config.get_config());
}