use crypto::digest::Digest;
use crypto::sha3::Sha3;
use uuid::Uuid;

pub trait Messagable {
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

#[derive(Clone, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub address: String,
    // this should be u16 but rust-postgres does not support it
    pub port: i32
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
}

impl Register {
    pub fn as_json(&self) -> String {
        json!({
            "address": self.address,
            "name": self.name,
            "port": self.port
        })
        .to_string()
    }
}

impl Messagable for Register {
    fn to_string(&self) -> String {
        let mut result = String::from("");
        result.push_str(self.address.as_str());
        result.push_str(self.name.as_str());
        result.push_str(self.port.to_string().as_str());
        result
    }
}