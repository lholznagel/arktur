use peers::Peer;

use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

const BASE_PATH: &str = "./peers";
const PATH_LAST_PEER: &str = "./peers/last_peer";

/// Manages the saving and retrieving of known peers
pub struct KnownPeers {
    peer: Peer
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
    /// - `peer` - Peer that should be saved
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
    pub fn get_latest() -> Peer {
        let file = File::open(PATH_LAST_PEER).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();

        if content != "" {
            let file = File::open(format!("peers/{}", content)).unwrap();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut content).unwrap();
            let mut lines = content.lines();

            Peer::new(String::from(lines.next().unwrap()), String::from(lines.next().unwrap()))
        } else {
            Peer::new(String::from(""), String::from(""))
        }
    }

    /// Gets a vector containing all known peers
    pub fn get_all() -> Vec<Peer> {
        let mut peers = Vec::new();

        if Path::new(BASE_PATH).exists() {
            let paths = fs::read_dir(BASE_PATH).unwrap();

            for path in paths {
                let mut content = String::new();
                let file = File::open(path.unwrap().path()).unwrap();
                let mut buf_reader = BufReader::new(file);
                buf_reader.read_to_string(&mut content).unwrap();
                let mut lines = content.lines();

                let name = lines.next();
                let socket = lines.next();

                if name != None && socket != None {
                    peers.push(Peer::new(String::from(name.unwrap()), String::from(socket.unwrap())));
                }
            }
        }

        peers
    }
}