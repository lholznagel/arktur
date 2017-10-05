use serde_yaml::from_str as load_from_str;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub database: Database,
    pub info: Info,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Info {
    pub name: String,
    pub address: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    pub address: String,
    pub password: String,
    pub port: u32,
    pub user: String,
}

impl Config {
    pub fn load() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut config_file_name = "config.yml";

        if args.len() > 1 && args[1] == "--config" {
            config_file_name = args[2].as_str();
        }

        let mut file = File::open(config_file_name).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).expect(
            "Could not read config",
        );

        load_from_str(&content.as_str()).unwrap()
    }
}