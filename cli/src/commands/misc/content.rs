use carina_core_protocol::{Events, MessageBuilder, Payload};
use carina_core_protocol::payloads::block::NewBlockContent;
use carina_core;
use carina_core::{Config, CarinaConfigBuilder};
use clap::ArgMatches;
use std::fs::File;
use std::io::Read;

pub fn execute(args: &ArgMatches) {
    let mut file = File::open(args.value_of("CONFIG").unwrap().to_string()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config: Config = match Config::from_str(&content) {
        Ok(val) => val,
        Err(e)  => panic!("[MISC_CONTENT] Error reading config file {:?}", e)
    };

    let carina_config_builder = CarinaConfigBuilder::new()
        .set_config(config);
    let (_, socket, config) = carina_core::init(carina_config_builder);

    let peers = {
        let state = config.lock().unwrap();
        state.config.peers.clone()
    };
    let mut nacl = {
        let state = config.lock().unwrap();
        state.config.nacl.clone()
    };

    for (_, peer) in &peers {
        let mut payload = NewBlockContent::new();
        payload.content = match args.value_of("CONTENT") {
            Some(val) => String::from(val),
            None      => String::new()
        };

        let message = MessageBuilder::new()
            .set_event_code(Events::as_val(Events::NewBlockContent))
            .set_payload(payload)
            .build(&mut nacl, &peer.public_key);

        match socket.send_to(&message, &peer.address) {
            Ok(_)  => debug!("[MISC_CONTENT] Added content"),
            Err(e) => error!("[MISC_CONTENT] Error adding content. {}", e),
        };
    }
}