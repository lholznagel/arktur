use base64::decode;
use failure::Error;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{PublicKey, SecretKey};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

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
    secret_key: SecretKey,
}

impl Config {
    /// creates a new instance of the config struct
    pub fn new(
        socket: String,
        peer_path: String,
        storage: String,
        uri: String,
        secret_key: String,
    ) -> Result<Self, Error> {
        let decoded: Vec<u8> = decode(&secret_key)?;
        let secret_key = SecretKey::from_slice(&decoded).expect("The secret key is not valid.");

        let mut config = Self {
            socket,
            peer_path,
            storage,
            uri,
            peers: Vec::new(),
            secret_key,
        };

        config.load_peers()?;
        Ok(config)
    }

    /// Loads the configuration from the given str
    ///
    /// # Params
    /// - `config` -> Yaml configuration as str
    ///
    /// # Return
    /// - `Result<Self, Error>` -> Config struct or error
    pub fn from_str(config: &str) -> Result<Self, Error> {
        let yaml = &YamlLoader::load_from_str(&config)?[0];

        let socket = yaml["socket"].as_str();
        let peers = yaml["peers"].as_str();
        let storage = yaml["storage"].as_str();
        let uri = yaml["uri"].as_str();
        let secret_key = yaml["uri"].as_str();

        if socket.is_some()
            && peers.is_some()
            && storage.is_some()
            && uri.is_some()
            && secret_key.is_some()
        {
            let decoded: Vec<u8> = decode(&secret_key.unwrap())?;
            let secret_key = SecretKey::from_slice(&decoded).expect("The secret key is not valid.");

            let mut config = Self {
                socket: socket.unwrap().to_string(),
                peer_path: peers.unwrap().to_string(),
                storage: storage.unwrap().to_string(),
                uri: uri.unwrap().to_string(),
                peers: Vec::new(),
                secret_key,
            };

            config.load_peers()?;
            Ok(config)
        } else {
            Err(format_err!("Config file not valid"))
        }
    }

    /// Loads the peer config file
    pub fn load_peers(&mut self) -> Result<(), Error> {
        let mut peers_storage = Vec::new();

        if Path::new(&self.peer_path).exists() {
            let mut file = File::open(self.peer_path.clone())?;
            let mut content = String::new();

            file.read_to_string(&mut content)?;
            let peer_file = YamlLoader::load_from_str(&content)?;

            for peers in peer_file {
                for peer in peers {
                    peers_storage.push(Peer::from_config_file(peer)?);
                }
            }
            self.peers = peers_storage;
        }
        Ok(())
    }
}

/// represents the peer config file
#[derive(Clone, Debug, PartialEq)]
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
    /// Loads peer information from the given yaml document
    pub fn from_config_file(config: Yaml) -> Result<Self, Error> {
        let address = config["address"].as_str();
        let public_key = config["public_key"].as_str();

        if address.is_some() && public_key.is_some() {
            Ok(Peer {
                // unwrap is save here -> validated
                address: address.unwrap().to_string(),
                public_key: public_key.unwrap().to_string(),
            })
        } else {
            Err(format_err!(
                "Error parsing address or public_key. Skipping peer."
            ))
        }
    }

    /// gets the public key of the peer
    pub fn public_key(self) -> Result<PublicKey, Error> {
        let decoded: Vec<u8> = decode(&self.public_key)?;
        Ok(PublicKey::from_slice(&decoded)
            .expect(&format!("The public key {} is not valid", self.public_key)))
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

        let mut deserialized = Vec::new();
        let peer_file = YamlLoader::load_from_str(&config_file).unwrap();
        for peers in peer_file {
            for peer in peers {
                deserialized.push(Peer::from_config_file(peer).unwrap());
            }
        }

        let peer_1 = Peer {
            address: "127.0.0.1:45002".to_string(),
            public_key: "OYGxJI79O18BFSCx3QUVNryww5v4i8qC85sdcx6N1SQ=".to_string(),
        };
        let peer_2 = Peer {
            address: "127.0.0.1:45003".to_string(),
            public_key: "/gfCzCrTj02YA+dAXCY2EODAYZFELeKH1bec5nenbU0=".to_string(),
        };

        assert_eq!(peer_1, deserialized[0]);
        assert_eq!(peer_2, deserialized[1]);
    }

    #[test]
    pub fn test_public_key() {
        let peer = Peer {
            address: "127.0.0.1:45002".to_string(),
            public_key: "OYGxJI79O18BFSCx3QUVNryww5v4i8qC85sdcx6N1SQ=".to_string(),
        };

        let expected = PublicKey::from_slice(&[
            57, 129, 177, 36, 142, 253, 59, 95, 1, 21, 32, 177, 221, 5, 21, 54, 188, 176, 195, 155,
            248, 139, 202, 130, 243, 155, 29, 115, 30, 141, 213, 36,
        ]).unwrap();
        assert_eq!(expected, peer.public_key().unwrap());
    }
}
