use carina_peer;
use carina_peer::config::Config;
use clap::ArgMatches;
use serde_yaml;
use std::fs::File;
use std::io::Read;

pub fn execute(args: &ArgMatches) {
    let mut file = File::open(args.value_of("CONFIG").unwrap().to_string()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config: Config = serde_yaml::from_str(&content).unwrap();

    carina_peer::init(config);
}