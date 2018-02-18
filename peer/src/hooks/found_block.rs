use blockchain_hooks::ApplicationState;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::FoundBlockPayload;

use hooks::State;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn on_found_block(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<FoundBlockPayload>::from_bytes(&state.payload_buffer).expect("Parsing the protocol should be successful");
    event!("FOUND_BLOCK {:?}", message.payload);

    create_folder();
    save_file(
        message.payload.index,
        message.payload.content, 
        message.payload.timestamp,
        message.payload.nonce,
        message.payload.prev,
        message.payload.hash);
}

fn create_folder() {
    if !Path::new("./block_data").exists() {
        fs::create_dir("./block_data").unwrap();
    }
}

fn save_file(index: u64, content: String, timestamp: i64, nonce: u64, prev: String, hash: String) {
    let mut filename = String::from("");

    for i in 0..16 {
        filename = filename + &hash.chars().nth(48 + i).unwrap().to_string();
    }

    let mut file = File::create(format!("{}/{}", "./block_data", filename))
        .expect("Could not write block file.");
    file.write_all(
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}", 
            index, 
            content, 
            timestamp, 
            nonce, 
            prev, 
            hash
        )
        .as_bytes())
    .expect("Error writing block information");
}