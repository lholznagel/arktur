use blockchain_hooks::{as_number, ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::{SyncBlocksAck, SyncBlocksReq};

use hooks::State;

use std::path::Path;

pub fn on_sync_blocks_ack(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<SyncBlocksAck>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    for block in message.payload.blocks {
        if !Path::new(&format!("{}/{}", state_lock.storage, block)).exists() {
            let payload = SyncBlocksReq {
                block
            };
            let message = BlockchainProtocol::new()
                .set_event_code(as_number(EventCodes::SyncBlocksReq))
                .set_payload(payload)
                .build();

            state.udp.send_to(message.as_slice(), state.source.clone())
                .expect("Sending using UDP should be successful.");
        }
    }
}