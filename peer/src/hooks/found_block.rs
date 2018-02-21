use blockchain_hooks::ApplicationState;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::FoundBlockPayload;

use hooks::State;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn on_found_block(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<FoundBlockPayload>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    info!("Saving new block to disk.");
    create_folder();
    save_file(message.payload);
}

fn create_folder() {
    if !Path::new("./block_data").exists() {
        fs::create_dir("./block_data").unwrap();
    }
}

fn save_file(block: FoundBlockPayload) {
    let mut filename = String::from("");

    for i in 0..16 {
        filename = filename + &block.hash.chars().nth(48 + i).unwrap().to_string();
    }

    let mut file = File::create(format!("{}/{}", "./block_data", filename))
        .expect("Could not create block file.");

    file.write_all(
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}", 
            block.index,
            block.content, 
            block.timestamp,
            block.nonce,
            block.prev,
            block.hash
        )
        .as_bytes())
        .expect("Error writing block information into file.");
}