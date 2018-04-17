use carina_peer;
use carina_peer::config::HolePuncher;
use clap::ArgMatches;
use configuration::Configuration;
use serde_yaml;
use std::fs::File;
use std::io::Read;

pub fn execute(args: &ArgMatches) {
    let mut file = File::open(args.value_of("CONFIG").unwrap().to_string()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config: Configuration = serde_yaml::from_str(&content).unwrap();

    let mut config = config.to_config();
    config.hole_puncher = HolePuncher {
        host: "0.0.0.0".to_string(),
        port: 50000
    };

    carina_peer::init(config);
}