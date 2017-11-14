use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};

const BASE_PATH: &str = "./peers";
const PATH_LAST_PEER: &str = "./peers/last_peer";

/// Contains all information about a peer
pub struct KnownPeers {
    /// Name of the peer
    pub name: String,
    /// Socket addr of the peer
    pub socket: String,
}

impl KnownPeers {
    /// Initializes all needed folders and files
    ///
    /// Should be called before all other methods
    pub fn init() {
        fs::create_dir(BASE_PATH).unwrap();
        File::create(PATH_LAST_PEER).unwrap();
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
    pub fn new(name: String, socket: String) -> Self {
        KnownPeers {
            name: name,
            socket: socket,
        }
    }

    /// Saves the peer information as file
    ///
    /// Besides that, the file `last_peer` is updated 
    ///
    /// # Returns
    ///
    /// Instance of iteself
    pub fn save(self) -> Self {
        let mut file = File::create(format!("peers/{}", self.name)).unwrap();
        file.write_all(format!("{}\n{}", self.name, self.socket).as_bytes()).unwrap();

        let mut file = File::create(PATH_LAST_PEER).unwrap();
        file.write_all(self.name.as_bytes()).unwrap();
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
                name: String::from(lines.next().unwrap()),
                socket: String::from(lines.next().unwrap()),
            }
        } else {
            KnownPeers {
                name: String::from(""),
                socket: String::from(""),
            }
        }
    }
}