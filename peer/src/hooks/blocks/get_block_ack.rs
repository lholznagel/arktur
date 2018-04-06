use carina_hooks::ApplicationState;
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::GetBlockAck;

use hooks::State;

use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn get_block_ack(state: ApplicationState<State>) {
    let nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };
    let source_peer = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.peers.get(&state.source.clone()).unwrap().clone()
    };

    let message = Protocol::<GetBlockAck>::from_bytes(&state.payload_buffer, &nacl, &source_peer.0)
        .expect("Parsing the protocol should be successful.");
    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    if !Path::new(&message.payload.filename).exists() {
        info!("Saving new block to disk.");
        let mut file = File::create(format!("{}/{}", state_lock.storage, message.payload.filename))
            .expect("Could not create block file.");

        let content = String::from(
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}", 
                message.payload.index,
                message.payload.content, 
                message.payload.timestamp,
                message.payload.nonce,
                message.payload.prev,
                message.payload.hash
            ));

        file.write_all(content.clone().as_bytes())
            .expect("Error writing block information into file.");
    }
}