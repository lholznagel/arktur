use blockchain_hooks::{as_number, ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::PossibleBlockPayload;
use blockchain_protocol::payload::blocks::HashVal;

use hooks::State;

pub fn on_possible_block(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<PossibleBlockPayload>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    info!("New possible block.");

    let payload = HashVal {
        content: message.payload.content,
        index: message.payload.index,
        nonce: message.payload.nonce,
        prev: message.payload.prev,
        timestamp: message.payload.timestamp
    };

    let message = BlockchainProtocol::new()
        .set_event_code(as_number(EventCodes::HashVal))
        .set_payload(payload)
        .build();

    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");
    for (peer, _) in state_lock.peers.clone() {
        state.udp.send_to(message.as_slice(), peer)
            .expect("Sending using UDP should be successful.");
    }
}