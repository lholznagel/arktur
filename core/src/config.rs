use base64::decode;
use failure::Error;
use serde_yaml;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{PublicKey, SecretKey};
use std::fs::File;
use std::io::Read;

/// Main configuration file
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    /// path to the socket file
    /// 
    /// # Example
    /// `/tmp/carina.sock`
    pub socket: String,
    /// path to the peers config file
    /// 
    /// # Example
    /// `./peers.yml`
    pub peer_path: String,
    /// block data storage location
    /// 
    /// # Example
    /// `./block_data`
    pub storage: String,
    /// port to listen
    /// 
    /// # Example
    /// `0.0.0.0:45000`
    pub uri: String,
    /// vector of all peers to connect
    pub peers: Vec<Peer>,
    /// secret key of the peer
    secret_key: SecretKey
}

impl Config {
    /// creates a new instance of the config struct
    pub fn new(
        socket: String,
        peer_path: String,
        storage: String,
        uri: String,
        secret_key: SecretKey,
    ) -> Result<Self, Error> {
        let mut file = File::open(peer_path.clone())?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;
        let peers = serde_yaml::from_str(&content)?;

        Ok(Self {
            socket,
            peer_path,
            storage,
            uri,
            peers,
            secret_key
        })
    }
}

/// represents the peer config file
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Peer {
    /// uri of the peer
    /// 
    /// # Example
    /// `0.0.0.0:45001`
    pub address: String,
    /// public key of the peer
    pub public_key: String,
}

impl Peer {
    /// gets the public key of the peer
    pub fn get_public_key(self) -> Result<PublicKey, Error> {
        let decoded: Vec<u8> = decode(&self.public_key)?;
        Ok(PublicKey::from_slice(&decoded).expect(&format!("The public key {} is not valid", self.public_key)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_config() {
        let config_file = r#"---
- address: 127.0.0.1:45002
  public_key: OYGxJI79O18BFSCx3QUVNryww5v4i8qC85sdcx6N1SQ=
- address: 127.0.0.1:45003
  public_key: /gfCzCrTj02YA+dAXCY2EODAYZFELeKH1bec5nenbU0="#;

        let deserialized: Vec<Peer> = serde_yaml::from_str(config_file).unwrap();

        let peer_1 = Peer {
            address: "127.0.0.1:45002".to_string(),
            public_key: "OYGxJI79O18BFSCx3QUVNryww5v4i8qC85sdcx6N1SQ=".to_string()
        };
        let peer_2 = Peer {
            address: "127.0.0.1:45003".to_string(),
            public_key: "/gfCzCrTj02YA+dAXCY2EODAYZFELeKH1bec5nenbU0=".to_string()
        };
        
        assert_eq!(peer_1, deserialized[0]);
        assert_eq!(peer_2, deserialized[1]);
    }
}