use crypto::digest::Digest;
use crypto::sha3::Sha3;
use uuid::Uuid;
use rocket_contrib::Value;

pub trait Messagable {
    fn as_json(&self) -> Value;
    fn to_string(&self) -> String;
}

#[derive(Clone, Deserialize, Debug)]
pub struct Message<T> {
    pub id: Uuid,
    pub timestamp: i64,
    pub content: T,
    pub hash: String,

    #[serde(skip_deserializing, skip_serializing)]
    pub is_valid_hash: bool,
}

impl<T: Messagable> Message<T> {
    pub fn get_hash(&self) -> String {
        let mut hash = String::from("");
        hash.push_str(&self.id.to_string().as_str());
        hash.push_str(&self.content.to_string());
        hash.push_str(&self.timestamp.to_string().as_str());

        let mut hasher = Sha3::sha3_256();
        hasher.input_str(hash.as_str());
        hasher.result_str()
    }

    pub fn validate_hash(mut self) -> Self {
        self.is_valid_hash = self.hash.clone() == self.get_hash();
        self
    }

    pub fn generate_hash(mut self) -> Self {
        self.hash = self.get_hash();
        self
    }
}

impl<T: Messagable> Messagable for Message<T> {
    fn as_json(&self) -> Value {
        json!({
            "content": self.content.as_json(),
            "id": self.id,
            "timestamp": self.timestamp,
            "hash": self.get_hash()
        })
    }

    fn to_string(&self) -> String {
        // this function is only for generating a hash
        // the message struct does not need to implement that
        unimplemented!();
    }
}