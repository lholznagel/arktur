use carina_hooks::{as_number, MessageState, HookCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::{GetBlock, GetBlockAck};

use hooks::State;

use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn get_block(state: MessageState<State>) {
    let mut nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };

    let message = Protocol::<GetBlock>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");
    let contacting_peer = state_lock.peers.get(&state.source.clone()).unwrap();

    if Path::new(&format!("{}/{}", state_lock.storage, message.payload.block)).exists() {
        let mut file = File::open(format!("{}/{}", state_lock.storage, message.payload.block)).expect("Should be able to read the block");
        let mut content = String::new();

        file.read_to_string(&mut content).expect("Should be able to read block");

        let result: Vec<&str> = content.split('\n').collect();

        let payload = GetBlockAck {
            filename: message.payload.block,
            index: result[0].to_string().parse::<u64>().unwrap(),
            content: result[1].to_string(),
            timestamp: result[2].to_string().parse::<i64>().unwrap(),
            nonce: result[3].to_string().parse::<u64>().unwrap(),
            prev: result[4].to_string(),
            hash: result[5].to_string()
        };

        let message = Protocol::new()
            .set_event_code(as_number(HookCodes::GetBlockAck))
            .set_payload(payload)
            .build(&mut nacl, &contacting_peer.0);

        state.udp.send_to(message.as_slice(), state.source.clone())
            .expect("Sending using UDP should be successful.");
    }
}