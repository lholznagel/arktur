use base64::decode;
use failure::Error;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{PublicKey, SecretKey};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

/// Parses the configuration files.
/// 
/// # Example config
/// ``` yaml
/// ---
/// socket: /tmp/carina.sock
/// peers: ./example_peers.yml
/// storage: ./block_data
/// uri: 0.0.0.0:45000
/// secret_key: W8TAQuFECexfADKJik6WBrh4G5qFaOhzX2eBZFIV8kY=
/// ```
/// 
/// # Example peers config
/// ``` yaml
/// ---
///- address: 127.0.0.1:45002
///  public_key: OYGxJI79O18BFSCx3QUVNryww5v4i8qC85sdcx6N1SQ=
///- address: 127.0.0.1:45003
///  public_key: /gfCzCrTj02YA+dAXCY2EODAYZFELeKH1bec5nenbU0=
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    /// path to the socket file
    pub socket: String,
    /// path to the peers config file
    pub peer_path: String,
    /// block data storage location
    pub storage: String,
    /// uri to listen on
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
        let secret_key = match SecretKey::from_slice(&decoded) {
            Some(v) => Ok(v),
            None => Err(format_err!("Invalid secret key"))
        }?;

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

        let socket = match yaml["socket"].as_str() {
            Some(v) => Ok(v),
            None    => Err(format_err!("Socket must be set"))
        }?.to_string();
        let peer_path = match yaml["peers"].as_str() {
            Some(v) => Ok(v),
            None    => Err(format_err!("Peers must be set"))
        }?.to_string();
        let storage = match yaml["storage"].as_str() {
            Some(v) => Ok(v),
            None    => Err(format_err!("Storage must be set"))
        }?.to_string();
        let uri = match yaml["uri"].as_str() {
            Some(v) => Ok(v),
            None    => Err(format_err!("Uri must be set"))
        }?.to_string();
        let secret_key = match yaml["secret_key"].as_str() {
            Some(v) => Ok(v),
            None    => Err(format_err!("Secret key must be set"))
        }?;

        let decoded: Vec<u8> = decode(&secret_key)?;
        let secret_key = match SecretKey::from_slice(&decoded) {
            Some(v) => Ok(v),
            None => Err(format_err!("Invalid secret key"))
        }?;

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

    /// Loads the peer config file and parses the peers
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

/// Represents the peer config file
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
    pub fn from_config_file(yaml: Yaml) -> Result<Self, Error> {
        let address = match yaml["address"].as_str() {
            Some(v) => Ok(v),
            None    => Err(format_err!("Address must be set"))
        }?.to_string();
        let public_key = match yaml["public_key"].as_str() {
            Some(v) => Ok(v),
            None    => Err(format_err!("Public key must be set"))
        }?.to_string();

        Ok(Peer {
            address,
            public_key,
        })
    }

    /// gets the public key of the peer
    pub fn public_key(self) -> Result<PublicKey, Error> {
        let decoded: Vec<u8> = decode(&self.public_key)?;
        match PublicKey::from_slice(&decoded) {
            Some(v) => Ok(v),
            None    => Err(format_err!("Invalid secret key"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_config() {
        let config_file = r#"---
socket: /tmp/carina.sock
peers: ""
storage: ./block_data
uri: 0.0.0.0:45000
secret_key: W8TAQuFECexfADKJik6WBrh4G5qFaOhzX2eBZFIV8kY="#;

        let config = Config::from_str(config_file).unwrap();

        let decoded: Vec<u8> = decode("W8TAQuFECexfADKJik6WBrh4G5qFaOhzX2eBZFIV8kY=").unwrap();
        let secret_key = SecretKey::from_slice(&decoded).unwrap();
        let expected = Config {
            socket: "/tmp/carina.sock".to_string(),
            peer_path: "".to_string(),
            storage: "./block_data".to_string(),
            uri: "0.0.0.0:45000".to_string(),
            peers: Vec::new(),
            secret_key,
        };

        assert_eq!(expected, config);
    }

    #[test]
    pub fn test_config_missing_storage() {
        let config_file = r#"---
socket: /tmp/carina.sock
peers: ""
uri: 0.0.0.0:45000
secret_key: W8TAQuFECexfADKJik6WBrh4G5qFaOhzX2eBZFIV8kY="#;

        assert!(Config::from_str(config_file).is_err(), true);
    }

    #[test]
    pub fn test_peer_config() {
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
