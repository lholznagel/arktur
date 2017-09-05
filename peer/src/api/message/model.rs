use crypto::digest::Digest;
use crypto::sha3::Sha3;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub content: String,
    pub timestamp: i64,

    #[serde(skip_serializing)]
    pub hash: String,
}

impl Message {
    pub fn is_valid_hash(self) -> bool {
        let mut hash_validation = String::from("");
        hash_validation.push_str(&self.id.to_string().as_str());
        hash_validation.push_str(&self.content.as_str());
        hash_validation.push_str(&self.timestamp.to_string().as_str());

        let mut hasher = Sha3::sha3_256();
        hasher.input_str(hash_validation.as_str());
        let hash = hasher.result_str();

        self.hash == hash
    }
}