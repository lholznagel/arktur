use toml;
use std::fs;
use std::fs::File;
use std::io::Write;
use models::config::Config;

pub fn create_dir(dir: &String) {
    match fs::create_dir(dir) {
        Ok(_) => {},
        Err(_) => panic!("Folder already exists.")
    };
}

pub fn save_config(config: Config) {
    write_file(String::from("config.toml"), toml::to_string(&config).unwrap());
}

pub fn write_file(filename: String, content: String) {
    match File::create(&filename) {
        Ok(mut file) => file.write_all(content.as_bytes()).unwrap(),
        Err(_) => panic!("{} Error during writing", &filename)
    }
}