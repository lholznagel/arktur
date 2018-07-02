use carina_core;
use carina_core::{CarinaConfigBuilder, Config};
use carina_core_protocol::payloads::block::CalcBlockPayload;
use carina_core_protocol::{Events, MessageBuilder};
use clap::ArgMatches;
use console::block_events::{BlockState, CalcBlock, NewBlockContent};
use console::misc_events::{Ping, Pong};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time as std_time;
use time;

pub fn execute(args: &ArgMatches) {
    let internal_state = Arc::new(Mutex::new(BlockState::new()));
    let mut content = String::new();

    // unwrap ok. CONFIG has a default value
    match File::open(args.value_of("CONFIG").unwrap().to_string()) {
        Ok(mut file) => match file.read_to_string(&mut content) {
            Ok(_) => (),
            Err(e) => panic!("[CONSOLE] Error readying config file. {}", e),
        },
        Err(e) => panic!("[CONSOLE] Error readying config file. {}", e),
    };

    let config: Config = match Config::from_str(&content) {
        Ok(val) => val,
        Err(e) => panic!("[CONSOLE] Error reading config file {:?}", e),
    };

    let carina_config_builder = CarinaConfigBuilder::new()
        .add_event(Events::Ping, Arc::new(Mutex::new(Ping {})))
        .add_event(Events::Pong, Arc::new(Mutex::new(Pong {})))
        .add_event(Events::CalcBlock, Arc::new(Mutex::new(CalcBlock::new())))
        .add_event(
            Events::NewBlockContent,
            Arc::new(Mutex::new(NewBlockContent::new(Arc::clone(
                &internal_state,
            )))),
        )
        .set_config(config);
    let (_, socket, config) = carina_core::init(carina_config_builder);

    let mut block_send = false;
    loop {
        let peers = {
            match config.lock() {
                Ok(val) => val.config.peers.clone(),
                Err(e) => {
                    error!("[MISC_CONTENT] Error locking state. {}", e);
                    HashMap::new()
                }
            }
        };
        let mut nacl = {
            let state = config.lock().unwrap();
            state.config.nacl.clone()
        };

        let current_time = time::now_utc();

        if current_time.tm_sec == 0 && current_time.tm_min % 2 == 0 && !block_send {
            debug!("[THREAD_CONSOLE] Time to generate a new block.");
            block_send = true;

            // TODO: Read out the database to find the real number of blocks
            let blocks_saved = 0;
            debug!("[THREAD_CONSOLE] Latest block number: {}", blocks_saved);

            let payload = CalcBlockPayload::block(0, String::from("0".repeat(64)), String::new());

            if blocks_saved > 0 {
                let mut next_block = String::new();

                let content = {
                    match internal_state.lock() {
                        Ok(val) => val.content.clone(),
                        Err(e) => {
                            error!("[THREAD_CONSOLE] Error locking state. {}", e);
                            HashMap::new()
                        }
                    }
                };

                for (_, content) in &content {
                    next_block.push_str(&content);
                }

                {
                    match internal_state.lock() {
                        Ok(mut val) => val.reset(),
                        Err(e) => error!("[THREAD_CONSOLE] Error locking state. {}", e),
                    }
                };

                // TODO: add save
                /*if Path::new(&format!("{}/last", state_lock.storage)).exists() {
                    let mut file = File::open(format!("{}/last", state_lock.storage))
                        .expect("Should be able to read the last block");
                    let mut content = String::new();

                    file.read_to_string(&mut content)
                        .expect("Should be able to read last block");

                    let result: Vec<&str> = content.split('\n').collect();
                    payload =
                        BlockGen::block(blocks_saved as u64 - 1, result[5].to_string(), next_block);
                    debug!("[THREAD BLOCK] Written block files");
                }*/            }

            for (_, peer) in peers.clone() {
                let message = MessageBuilder::new()
                    .set_event_code(Events::as_val(Events::Ping))
                    .set_payload(payload.clone())
                    .build(&mut nacl, &peer.public_key);

                match socket.send_to(&message, &peer.address) {
                    Ok(_) => debug!("[THREAD_CONSOLE] Send calc_block to {}", peer.address),
                    Err(e) => error!(
                        "[THREAD_CONSOLE] Error sending calc_block to peer: {}. Error: {}",
                        peer.address, e
                    ),
                };
            }
        } else {
            thread::sleep(std_time::Duration::from_secs(
                (60 - current_time.tm_sec) as u64,
            ));
            block_send = false;
        }
    }
}
