use std::fs::File;
use std::io::Read;
use serde_yaml::from_str as load_from_str;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub info: Info,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub address: String,
    pub password: String,
    pub port: u32,
    pub user: String,
}

impl Config {
    pub fn new() -> Self {
        let mut file = File::open("config.yml").unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).expect(
            "Could not read config",
        );

        load_from_str(&content.as_str()).unwrap()
    }
}