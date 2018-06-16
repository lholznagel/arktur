use carina_core_protocol::{MessageBuilder, Payload};
use carina_core_protocol::payloads::EmptyPayload;
use carina_core;
use carina_core::{Config, CarinaConfigBuilder, Events};
use clap::ArgMatches;
use commands::ping::Pong;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn execute(args: &ArgMatches) {
    let mut file = File::open(args.value_of("CONFIG").unwrap().to_string()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config: Config = match Config::from_str(&content) {
        Ok(val) => val,
        Err(e)  => panic!("[MISC_PING] Error reading config file {:?}", e)
    };

    let pong_event = Arc::new(Mutex::new(Pong::new()));
    let carina_config_builder = CarinaConfigBuilder::new()
        .add_event(Events::Pong, Arc::clone(&pong_event))
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
        let message = MessageBuilder::new()
            .set_event_code(0)
            .set_payload(EmptyPayload::new())
            .build(&mut nacl, &peer.public_key);

        match socket.send_to(&message, &peer.address) {
            Ok(_)  => debug!("[MISC_PING] Send ping to peer {}", peer.address),
            Err(e) => error!("[MISC_PING] Error sending ping to peer: {}. Error: {}", peer.address, e),
        };
    }

    info!("[MISC_PING] Waiting 30 seconds.");
    thread::sleep(Duration::from_secs(30));
    {
        let event = pong_event.lock().unwrap();
        info!("[MISC_PING] Following peers answered {:?}", event.answered);
    }
}