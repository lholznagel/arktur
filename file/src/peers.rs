use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};

pub struct KnownPeers {
    pub name: String,
    pub socket: String,
}

impl KnownPeers {
    pub fn init() {
        fs::create_dir("./peers").unwrap();
        File::create("./peers/latest_peer").unwrap();
    }

    pub fn new(name: String, socket: String) -> Self {
        KnownPeers {
            name: name,
            socket: socket,
        }
    }

    pub fn save(self) -> Self {
        let mut file = File::create(format!("peers/{}", self.name)).unwrap();
        file.write_all(format!("{}\n{}", self.name, self.socket).as_bytes()).unwrap();

        let mut file = File::create("peers/latest_peer").unwrap();
        file.write_all(self.name.as_bytes()).unwrap();
        self
    }

    pub fn get_latest() -> Self {
        let file = File::open("peers/latest_peer").unwrap();
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