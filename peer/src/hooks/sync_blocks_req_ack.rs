use blockchain_hooks::ApplicationState;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::SyncBlocksReqAck;

use hooks::State;

use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn on_sync_blocks_req_ack(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<SyncBlocksReqAck>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    if !Path::new(&message.payload.filename).exists() {
        info!("Saving new block to disk.");
        let mut file = File::create(format!("{}/{}", "./block_data", message.payload.filename))
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