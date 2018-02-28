use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::SyncBlocksAck;

use hooks::State;

use std::fs::read_dir;

pub fn on_sync_blocks(state: ApplicationState<State>) {
    info!("Syncing blocks.");
    let mut count = 0;
    let mut blocks = Vec::new();

    for path in read_dir("./block_data").expect("Should be able to read path.") {
        let path = String::from(path.unwrap().path().file_name().unwrap().to_str().unwrap());
        blocks.push(path);

        if count == 999 {
            let payload = SyncBlocksAck {
                blocks
            };
            let message = BlockchainProtocol::new()
                .set_event_code(EventCodes::SyncBlocksAck)
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
        let payload = SyncBlocksAck {
            blocks
        };
        let message = BlockchainProtocol::new()
            .set_event_code(EventCodes::SyncBlocksAck)
            .set_payload(payload)
            .build();

        state.udp.send_to(message.as_slice(), state.source.clone())
            .expect("Sending using UDP should be successful.");
    }
}