use carina_core_protocol::Events;
use carina_core;
use carina_core::{Config, CarinaConfigBuilder};
use clap::ArgMatches;
use commands::console::events::{Ping, Pong, NewBlockContent};
use commands::console::InternalState;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

pub fn execute(args: &ArgMatches) {
    let internal_state = Arc::new(Mutex::new(InternalState::new()));
    let mut content = String::new();

    // unwrap ok. CONFIG has a default value
    match File::open(args.value_of("CONFIG").unwrap().to_string()) {
        Ok(mut file) => match file.read_to_string(&mut content) {
            Ok(_)  => (),
            Err(e) => panic!("[CONSOLE] Error readying config file. {}", e)
        },
        Err(e)     => panic!("[CONSOLE] Error readying config file. {}", e)
    };

    let config: Config = match Config::from_str(&content) {
        Ok(val) => val,
        Err(e)  => panic!("[CONSOLE] Error reading config file {:?}", e)
    };

    let carina_config_builder = CarinaConfigBuilder::new()
        .add_event(Events::Ping, Arc::new(Mutex::new(Ping{})))
        .add_event(Events::Pong, Arc::new(Mutex::new(Pong{})))
        .add_event(Events::NewBlockContent, Arc::new(Mutex::new(NewBlockContent::new(Arc::clone(&internal_state)))))
        .set_config(config);
    let (thread, _, _) = carina_core::init(carina_config_builder);

    thread.join().unwrap();
}