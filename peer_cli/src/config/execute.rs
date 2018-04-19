use clap::ArgMatches;
use configuration::Configuration;
use serde_yaml;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn execute(args: &ArgMatches) {
    if args.is_present("init") {

        if !Path::new("config.yml").exists() {
            let config = Configuration::new();
            let mut file = File::create("config.yml").unwrap();
            let parsed = serde_yaml::to_string(&config).unwrap();
            file.write_all(parsed.as_bytes()).unwrap();
        } else {
            println!("Config file already exists");
        }
    }
}