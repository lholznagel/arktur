use crypto::digest::Digest;
use crypto::sha3::Sha3;
use time::get_time;

#[derive(Debug)]
pub struct Block {
    id: u16,
    nonce: u32,
    content: String,
    timestamp: i64,
    prev: String,
    hash: String,
}

impl Block {
    pub fn new(id: u16, content: String, prev: String) -> Self {
        Block {
            id: id,
            nonce: 0,
            content: content,
            timestamp: get_time().sec,
            prev: prev,
            hash: String::from(""),
        }
    }

    pub fn generate_hash(mut self) -> Self {
        loop {
            let mut current = String::from("");
            current.push_str(self.id.to_string().as_str());
            current.push_str(self.nonce.to_string().as_str());
            current.push_str(self.content.as_str());
            current.push_str(self.prev.as_str());

            let mut hasher = Sha3::sha3_256();
            hasher.input_str(current.as_str());
            let hex = hasher.result_str();

            if "0000" == &hex[..4] {
                self.hash = hex.clone();
                break;
            } else {
                self.nonce = self.nonce + 1;
            }
        }

        self
    }
}