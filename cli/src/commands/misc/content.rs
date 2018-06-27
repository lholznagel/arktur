use carina_core_protocol::{Events, MessageBuilder, Payload};
use carina_core_protocol::payloads::block::NewBlockContent;
use carina_core;
use carina_core::{Config, CarinaConfigBuilder};
use clap::ArgMatches;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn execute(args: &ArgMatches) {
    let mut content = String::new();

    // unwrap ok. CONFIG has a default value
    match File::open(args.value_of("CONFIG").unwrap().to_string()) {
        Ok(mut file) => match file.read_to_string(&mut content) {
            Ok(_)  => (),
            Err(e) => panic!("[MISC_CONTENT] Error readying config file. {}", e)
        },
        Err(e)     => panic!("[MISC_CONTENT] Error readying config file. {}", e)
    };

    let config: Config = match Config::from_str(&content) {
        Ok(val) => val,
        Err(e)  => panic!("[MISC_CONTENT] Error reading config file {:?}", e)
    };

    let carina_config_builder = CarinaConfigBuilder::new()
        .set_config(config);
    let (_, socket, config) = carina_core::init(carina_config_builder);

    let peers = {
        match config.lock() {
            Ok(val) => val.config.peers.clone(),
            Err(e)  => {
                error!("[MISC_CONTENT] Error locking state. {}", e);
                HashMap::new()
            }
        }
    };
    let mut nacl = {
        let state = config.lock().unwrap();
        state.config.nacl.clone()
    };

    let mut payload = NewBlockContent::new();
    // save, because it is forced by clap
    payload.content = String::from(args.value_of("CONTENT").unwrap());

    for (_, peer) in &peers {
        let message = MessageBuilder::new()
            .set_event_code(Events::as_val(Events::NewBlockContent))
            .set_payload(payload.clone())
            .build(&mut nacl, &peer.public_key);

        match socket.send_to(&message, &peer.address) {
            Ok(_)  => debug!("[MISC_CONTENT] Added content"),
            Err(e) => error!("[MISC_CONTENT] Error adding content. {}", e),
        };
    }
}