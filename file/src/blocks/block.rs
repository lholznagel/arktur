use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const BASE_PATH: &str = "./block_data";

/// Represents a block
#[derive(Debug, PartialEq)]
pub struct Block {
    /// Block index
    pub index: u64,
    /// Content that is saved in the block
    pub content: String,
    /// Timestamp this block was generated
    pub timestamp: i64,
    /// Nonce that was needed for the hash
    pub nonce: u64,
    /// Hash of the previous block
    pub prev: String,
    /// The hash of the block
    pub hash: String
}

impl Block {
    /// Creates all needed folders if they don´t exist
    pub fn init() {
        if !Path::new(BASE_PATH).exists() {
            fs::create_dir(BASE_PATH).unwrap();
        }
    }

    /// Creates a new block instance
    ///
    /// # Return
    /// - `block` - Instance of an empty block
    pub fn new() -> Self {
        Block {
            index: 0,
            content: String::from(""),
            timestamp: 0,
            nonce: 0,
            prev: String::from(""),
            hash: String::from("")
        }
    }

    /// Saves the current block
    pub fn save(&self) {
        let mut filename = String::from("");

        for i in 0..16 {
            filename = filename + &self.hash.chars().nth(48 + i).unwrap().to_string();
        }

        let mut file = File::create(format!("{}/{}", BASE_PATH, filename)).unwrap();
        file.write_all(format!("{}\n{}\n{}\n{}\n{}\n{}", self.index, self.content, self.timestamp, self.nonce, self.prev, self.hash).as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Read};

    #[test]
    fn test_is_file_generated() {
        let hash_prev = "dd8771ca07eb1bbbec4eb99745f61d3e9e609718f2c94bb24a62eaa092e300a1";
        let hash_current = "a7b3e78c5a4e17c2b187d98751f41d15b9a9f5bf460f4d1a1f9524175206f4d7";
        let hash_filename = "1f9524175206f4d7";

        if !Path::new(BASE_PATH).exists() {
            fs::create_dir(BASE_PATH).unwrap();
        }

        let mut block = Block::new();
        block.index = 100;
        block.content = String::from("Some content");
        block.timestamp = 0;
        block.nonce = 1000;
        block.prev = hash_prev.to_string();
        block.hash = hash_current.to_string();
        block.save();

        assert_eq!(true, Path::new(&format!("{}/{}", BASE_PATH, hash_filename)).exists());

        fs::remove_file(&format!("{}/{}", BASE_PATH, hash_filename)).unwrap();
    }

    #[test]
    fn test_validate_file_content() {
        let hash_prev = "b9815bbae48b332e29053091bf919ce7482a7f24f27a99f9bb248c3854cc6193";
        let hash_current = "5dd42db62ac60139dbdc00e0505913d4060e2cac186b71dfcdd31958cba4fca5";
        let hash_filename = "cdd31958cba4fca5";

        if !Path::new(BASE_PATH).exists() {
            fs::create_dir(BASE_PATH).unwrap();
        }

        let mut block = Block::new();
        block.index = 100;
        block.content = String::from("Some content");
        block.timestamp = 0;
        block.nonce = 1000;
        block.prev = hash_prev.to_string();
        block.hash = hash_current.to_string();
        block.save();

        let file = File::open(format!("{}/{}", BASE_PATH, hash_filename)).unwrap();
        let mut content = String::from("");
        let mut buf_reader = BufReader::new(file);
        buf_reader.read_to_string(&mut content).unwrap();
        let mut lines = content.lines();

        let mut read_file = Block::new();
        read_file.index = String::from(lines.next().unwrap()).parse::<u64>().unwrap();
        read_file.content = String::from(lines.next().unwrap());
        read_file.timestamp = String::from(lines.next().unwrap()).parse::<i64>().unwrap();
        read_file.nonce = String::from(lines.next().unwrap()).parse::<u64>().unwrap();
        read_file.prev = String::from(lines.next().unwrap());
        read_file.hash = String::from(lines.next().unwrap());

        assert_eq!(block, read_file);

        fs::remove_file(&format!("{}/{}", BASE_PATH, hash_filename)).unwrap();
    }
}