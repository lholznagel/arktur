use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::GetBlocksAck;

use hooks::State;

use std::fs::read_dir;

pub fn get_blocks(state: ApplicationState<State>) {
    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    info!("Syncing blocks.");
    let mut count = 0;
    let mut blocks = Vec::new();

    for path in read_dir(&state_lock.storage).expect("Should be able to read path.") {
        let path = String::from(path.unwrap().path().file_name().unwrap().to_str().unwrap());
        blocks.push(path);

        if count == 999 {
            let payload = GetBlocksAck {
                blocks
            };
            let message = Protocol::new()
                .set_event_code(as_number(EventCodes::GetBlocksAck))
                .set_payload(payload)
                .build();

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
            .set_event_code(as_number(EventCodes::GetBlocksAck))
            .set_payload(payload)
            .build();

        state.udp.send_to(message.as_slice(), state.source.clone())
            .expect("Sending using UDP should be successful.");
    }
}