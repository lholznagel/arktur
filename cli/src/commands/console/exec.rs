use carina_core;
use carina_core::{Config, CarinaConfigBuilder, Events};
use clap::ArgMatches;
use commands::console::events::{Ping, Pong};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

pub fn execute(args: &ArgMatches) {
    let mut file = File::open(args.value_of("CONFIG").unwrap().to_string()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config: Config = match Config::from_str(&content) {
        Ok(val) => val,
        Err(e)  => panic!("[CONSOLE] Error reading config file {:?}", e)
    };

    let carina_config_builder = CarinaConfigBuilder::new()
        .add_event(Events::Ping, Arc::new(Mutex::new(Ping{})))
        .add_event(Events::Pong, Arc::new(Mutex::new(Pong{})))
        .set_config(config);
    let (thread, _, _) = carina_core::init(carina_config_builder);

    thread.join().unwrap();
}