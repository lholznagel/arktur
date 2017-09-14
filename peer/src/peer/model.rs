use message::Messagable;
use uuid::Uuid;
use rocket_contrib::Value;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Register {
    pub address: String,
    pub name: String,
    // this should be u16 but rust-postgres does not support it
    pub port: i32,
    pub peer_id: Uuid,
    pub notify_on_change: bool
}

impl Messagable for Register {
    fn as_json(&self) -> Value {
        json!({
            "address": self.address,
            "name": self.name,
            "port": self.port,
            "peer_id": self.peer_id,
            "notify_on_change": self.notify_on_change
        })
    }

    fn to_string(&self) -> String {
        let mut result = String::from("");
        result.push_str(self.address.as_str());
        result.push_str(self.name.as_str());
        result.push_str(self.port.to_string().as_str());
        result.push_str(self.peer_id.to_string().as_str());
        result.push_str(self.notify_on_change.to_string()
        .as_str());
        result
    }
}