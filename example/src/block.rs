use crypto::digest::Digest;
use crypto::sha3::Sha3;
use time::get_time;

#[derive(Debug)]
pub struct Block {
    pub index: usize,
    pub nonce: u32,
    pub content: String,
    pub timestamp: i64,
    pub prev: String,
    pub hash: String,
}

impl Block {
    pub fn new(content: String, prev: String) -> Self {
        Block {
            index: 0,
            nonce: 0,
            content: content,
            timestamp: get_time().sec,
            prev: prev,
            hash: String::from(""),
        }
    }

    pub fn set_index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn generate_hash(mut self, signkey: String) -> Self {
        loop {
            let mut current = String::from("");
            current.push_str(self.index.to_string().as_str());
            current.push_str(self.nonce.to_string().as_str());
            current.push_str(self.content.as_str());
            current.push_str(self.prev.as_str());

            let mut hasher = Sha3::sha3_256();
            hasher.input_str(current.as_str());
            let hex = hasher.result_str();

            if signkey == &hex[..signkey.len()] {
                self.hash = hex.clone();
                break;
            } else {
                self.nonce = self.nonce + 1;
            }
        }

        self
    }
}