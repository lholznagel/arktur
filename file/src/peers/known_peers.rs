use peers::Peer;

use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};

const BASE_PATH: &str = "./peers";
const PATH_LAST_PEER: &str = "./peers/last_peer";

/// Manages the saving and retrieving of known peers
pub struct KnownPeers {
    /// Instance of peer
    pub peer: Peer
}

impl KnownPeers {
    /// Initializes all needed folders and files
    ///
    /// Should be called before all other methods
    pub fn init() {
        if !Path::new(BASE_PATH).exists() {
            fs::create_dir(BASE_PATH).unwrap();
            File::create(PATH_LAST_PEER).unwrap();
        }
    }

    /// Removes the peers folder
    pub fn clean() {
        fs::remove_dir(BASE_PATH).unwrap();
    }

    /// Creates a new instance
    ///
    /// # Parameters
    ///
    /// - `name` - Name of the peer
    /// - `socket` - Socket of the peer
    ///
    /// # Return
    ///
    /// Instance of itself
    pub fn new(peer: Peer) -> Self {
        KnownPeers {
            peer: peer
        }
    }

    /// Saves the peer information as file
    ///
    /// Besides that, the file `last_peer` is updated 
    ///
    /// # Returns
    ///
    /// Instance of iteself
    pub fn save(mut self) -> Self {
        self.peer = self.peer.save();

        let mut file = File::create(PATH_LAST_PEER).unwrap();
        file.write_all(self.peer.get_name().as_bytes()).unwrap();
        self
    }

    /// Gets an instance of this struct containing the information
    /// from the last peer that registered itself
    ///
    /// # Return
    ///
    /// Instance of itself, containing the peer information
    pub fn get_latest() -> Self {
        let file = File::open(PATH_LAST_PEER).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();

        if content != "" {
            let file = File::open(format!("peers/{}", content)).unwrap();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut content).unwrap();
            let mut lines = content.lines();

            KnownPeers {
                peer: Peer::new(String::from(lines.next().unwrap()), String::from(lines.next().unwrap()))
            }
        } else {
            KnownPeers {
                peer: Peer::new(String::from(""), String::from(""))
            }
        }
    }
}