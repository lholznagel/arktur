use crypto::digest::Digest;
use crypto::sha3::Sha3;
use uuid::Uuid;

pub trait Messegable {
    fn model_as_str(&self) -> String;
}

#[derive(Clone, Deserialize)]
pub struct Message<T: Messegable> {
    pub id: Uuid,
    pub timestamp: i64,
    pub content: T,

    #[serde(skip_serializing)]
    pub hash: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub is_valid_hash: bool,
}

#[derive(Clone, Deserialize)]
pub struct Register {
    pub name: String
}

impl <T: Messegable> Message<T> {
    pub fn validate_hash(mut self) -> Self {
        let mut hash_validation = String::from("");
        hash_validation.push_str(&self.id.to_string().as_str());
        hash_validation.push_str(&self.content.model_as_str());
        hash_validation.push_str(&self.timestamp.to_string().as_str());

        let mut hasher = Sha3::sha3_256();
        hasher.input_str(hash_validation.as_str());
        let hash = hasher.result_str();
        println!("{:?}", hash);

        self.is_valid_hash = self.hash.clone() == hash;
        self
    }
}

impl Messegable for Register {
    fn model_as_str(&self) -> String {
        let mut result = String::from("");
        result.push_str(&self.name.as_str());
        result
    }
}