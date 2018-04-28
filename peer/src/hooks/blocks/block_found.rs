use carina_hooks::MessageState;
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::BlockFound;

use hooks::State;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn block_found(state: MessageState<State>) {
    let message = Protocol::<BlockFound>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");

        let path = format!("{}/{}", env::var("PWD").unwrap(), &state_lock.storage);
        if !Path::new(&path).exists() {
            fs::create_dir(&path).expect("PATH::new");
        }
    }

    save_file(message.payload, state);
}

fn save_file(block: BlockFound, state: MessageState<State>) {
    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    let mut filename = String::from("");

    for i in 0..16 {
        filename = filename + &block.hash.chars().nth(48 + i).expect("Filename").to_string();
    }

    if !Path::new(&filename).exists() {
        info!("Saving new block to disk.");
        let mut file = File::create(format!("{}/{}", state_lock.storage, filename))
            .expect("Could not create block file.");
        let mut file_last = File::create(format!("{}/last", state_lock.storage))
            .expect("Could not create block file.");

        let content = String::from(
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}", 
                block.index,
                block.content, 
                block.timestamp,
                block.nonce,
                block.prev,
                block.hash
            ));

        file.write_all(content.clone().as_bytes())
            .expect("Error writing block information into file.");
        file_last.write_all(content.clone().as_bytes())
            .expect("Error writing block information into file.");
    }
}