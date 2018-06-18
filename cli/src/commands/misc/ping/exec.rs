use carina_core_protocol::{MessageBuilder, Payload};
use carina_core_protocol::payloads::EmptyPayload;
use carina_core;
use carina_core::{Config, CarinaConfigBuilder, Events};
use clap::ArgMatches;
use commands::misc::ping::Pong;
use prettytable::{Attr, color, Table};
use prettytable::cell::Cell;
use prettytable::row::Row;
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

    let pong_event = Arc::new(Mutex::new(Pong::new(config.peers.clone())));
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
        match pong_event.lock() {
            Ok(event) => {
                let mut table = Table::new();
                table.add_row(row!["Address", "Status"]);

                for (key, value) in &event.answered {
                    let row = match value {
                        true => Row::new(vec![Cell::new(key), Cell::new("OK").with_style(Attr::ForegroundColor(color::GREEN))]),
                        false => Row::new(vec![Cell::new(key), Cell::new("No response").with_style(Attr::ForegroundColor(color::RED))])
                    };
                    table.add_row(row);
                }

                table.printstd();
                ()
            },
            Err(_)    => error!("[MISC_PING] Error locking mutex.")
        };
    }
}