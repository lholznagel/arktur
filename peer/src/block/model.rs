use crypto::digest::Digest;
use crypto::sha3::Sha3;
use time::get_time;
use uuid::Uuid;

#[derive(Debug)]
pub struct ResultLastBlock {
    pub index: i32,
    pub prev: String,
}

#[derive(Deserialize)]
pub struct BlockApi {
    pub content: String,
    pub blockchain: Uuid,
}

#[derive(Debug)]
pub struct Block {
    pub index: i32,
    pub nonce: i32,
    pub content: String,
    pub timestamp: i64,
    pub prev: String,
    pub hash: String,
    pub blockchain: Uuid,
}

impl Block {
    pub fn new(index: i32, content: String, prev: String, blockchain: Uuid) -> Self {
        Block {
            index: index,
            nonce: 0,
            content: content,
            timestamp: get_time().sec,
            prev: prev,
            hash: String::from(""),
            blockchain: blockchain
        }
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