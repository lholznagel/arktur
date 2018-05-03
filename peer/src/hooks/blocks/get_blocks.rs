use carina_hooks::{as_number, MessageState, HookCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::GetBlocksAck;

use hooks::State;

use std::fs::read_dir;

pub fn get_blocks(state: MessageState<State>) {
    let mut nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };
    let peers = {
        let state_lock = state.state.lock().expect("Locking the mutex should be successful.");
        state_lock.peers.clone()
    };
    let storage = {
        let state_lock = state.state.lock().expect("Locking the mutex should be successful.");
        state_lock.storage.clone()
    };

    let contacting_peer = peers.get(&state.source.clone()).unwrap();

    info!("Syncing blocks.");
    let mut count = 0;
    let mut blocks = Vec::new();

    for path in read_dir(&storage).expect("Should be able to read path.") {
        let path = String::from(path.unwrap().path().file_name().unwrap().to_str().unwrap());
        blocks.push(path);

        if count == 999 {
            let payload = GetBlocksAck {
                blocks
            };
            let message = Protocol::new()
                .set_event_code(as_number(HookCodes::GetBlocksAck))
                .set_payload(payload)
                .build(&mut nacl, &contacting_peer.0);

            state.udp.send_to(message.as_slice(), state.source.clone())
                .expect("Sending using UDP should be successful.");

            count = 0;
            blocks = Vec::new();
        }

        count += 1;
    }

    if blocks.len() > 0 {
        let payload = GetBlocksAck {
            blocks
        };
        let message = Protocol::new()
            .set_event_code(as_number(HookCodes::GetBlocksAck))
            .set_payload(payload)
            .build(&mut nacl, &contacting_peer.0);

        state.udp.send_to(message.as_slice(), state.source.clone())
            .expect("Sending using UDP should be successful.");
    }
}