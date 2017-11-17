use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const BASE_PATH: &str = "./peers";

/// Contains all information about a peer
pub struct Peer {
    name: String,
    socket: String,
}

impl Peer {
    /// Init all needed paths
    pub fn init() {
        if !Path::new(BASE_PATH).exists() {
            fs::create_dir(BASE_PATH).unwrap();
        }
    }

    /// Creates a new peer instance
    ///
    /// # Parameters
    ///
    /// - `name` - Name of the peer
    /// - `socket` - Socket of the peer
    ///
    /// # Returns
    ///
    /// New peer instance
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_file::peers::Peer;
    ///
    /// let peer = Peer::new(String::from("SomePeer"), String::from("127.0.0.1:1337"));
    /// // do something with peer
    /// ```
    pub fn new(name: String, socket: String) -> Self {
        Peer {
            name: name,
            socket: socket
        }
    }

    /// Saved the peer information
    pub fn save(self) -> Self {
        let mut file = File::create(format!("peers/{}", self.name)).unwrap();
        file.write_all(format!("{}\n{}", self.name, self.name).as_bytes()).unwrap();
        self
    }

    /// Gets the name of the peer
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// Gets the socket of the peer
    pub fn get_socket(&self) -> &str {
        self.socket.as_str()
    }
}