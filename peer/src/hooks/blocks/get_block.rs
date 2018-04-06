use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::{GetBlock, GetBlockAck};

use hooks::State;

use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn get_block(state: ApplicationState<State>) {
    let message = Protocol::<GetBlock>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

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
            .set_event_code(as_number(EventCodes::GetBlockAck))
            .set_payload(payload)
            .build(&state_lock.nacl);

        state.udp.send_to(message.as_slice(), state.source.clone())
            .expect("Sending using UDP should be successful.");
    }
}