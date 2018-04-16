use carina_peer::config::Config;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub port: u16,
    pub storage: String,
}

impl Configuration {
    pub fn to_config(self) -> Config {
        let mut config = Config::new();
        config.port = self.port;
        config.storage = self.storage;
        config
    }
}