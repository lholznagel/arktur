use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::{GetBlocksAck, GetBlock};

use hooks::State;

use std::path::Path;

pub fn get_blocks_ack(state: ApplicationState<State>) {
    let mut nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };
    let source_peer = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.peers.get(&state.source.clone()).unwrap().clone()
    };

    let message = Protocol::<GetBlocksAck>::from_bytes(&state.payload_buffer, &nacl, &source_peer.0)
        .expect("Parsing the protocol should be successful.");

    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");
    let contacting_peer = state_lock.peers.get(&state.source.clone()).unwrap();

    for block in message.payload.blocks {
        if !Path::new(&format!("{}/{}", state_lock.storage, block)).exists() {
            let payload = GetBlock {
                block
            };
            let message = Protocol::new()
                .set_event_code(as_number(EventCodes::GetBlock))
                .set_payload(payload)
                .build(&mut nacl, &contacting_peer.0);

            state.udp.send_to(message.as_slice(), state.source.clone())
                .expect("Sending using UDP should be successful.");
        }
    }
}