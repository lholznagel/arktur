use carina_hooks::{as_number, MessageState, HookCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::Payload;
use carina_protocol::payload::blocks::{BlockFound, HashValAck};

use hooks::State;

use std::collections::HashMap;

pub fn hash_val_ack(state: MessageState<State>) {
    let mut nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };

    let message = Protocol::<HashValAck>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    let mut state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    state_lock.hashes.push(message.payload.hash);

    if state_lock.hashes.len() == state_lock.peers.len() {
        let mut hashes = HashMap::new();

        for hash in state_lock.hashes.clone() {
            let updated_value = match hashes.get(&hash) {
                Some(current_val)   => current_val + 1,
                None                => 1
            };

            hashes.insert(hash, updated_value);
        }

        let mut result: (String, u64) = (String::from(""), 0);
        for (key, value) in hashes {
            if result.1 == 0 || value > result.1 {
                result.0 = key;
                result.1 = value;
            }
        }

        state_lock.hashes = Vec::new();
        state_lock.current_block.hash = result.0;

        let mut payload = BlockFound::new();
        payload.content = state_lock.current_block.content.clone();
        payload.index = state_lock.current_block.index;
        payload.nonce = state_lock.current_block.nonce;
        payload.prev = state_lock.current_block.prev.clone();
        payload.timestamp = state_lock.current_block.timestamp;
        payload.hash = state_lock.current_block.hash.clone();

        for (peer, (public_key, _, _)) in state_lock.peers.clone() {
            let message = Protocol::new()
                .set_event_code(as_number(HookCodes::BlockFound))
                .set_payload(payload.clone())
                .build(&mut nacl, &public_key);

            state.udp.send_to(message.as_slice(), peer).unwrap();
        }
    }
}