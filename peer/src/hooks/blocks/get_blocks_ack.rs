use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::{GetBlocksAck, GetBlock};

use hooks::State;

use std::path::Path;

pub fn get_blocks_ack(state: ApplicationState<State>) {
    let message = Protocol::<GetBlocksAck>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    for block in message.payload.blocks {
        if !Path::new(&format!("{}/{}", state_lock.storage, block)).exists() {
            let payload = GetBlock {
                block
            };
            let message = Protocol::new()
                .set_event_code(as_number(EventCodes::GetBlock))
                .set_payload(payload)
                .build();

            state.udp.send_to(message.as_slice(), state.source.clone())
                .expect("Sending using UDP should be successful.");
        }
    }
}