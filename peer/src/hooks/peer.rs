use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{Payload, RegisterAckPayload};

use hooks::State;

pub fn on_register_peer(state: ApplicationState<State>) {
    let mut state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    if state_lock.peers.is_empty() {
        let answer = BlockchainProtocol::new()
            .set_event_code(EventCodes::RegisterPeerAck)
            .set_status_code(StatusCodes::NoPeer)
            .set_payload(RegisterAckPayload::new())
            .build();
        state.udp.send_to(&answer, state.source.clone())
            .expect("Sending using UDP should be successful.");
    } else {
        let answer = BlockchainProtocol::new()
            .set_event_code(EventCodes::RegisterPeerAck)
            .set_status_code(StatusCodes::Ok)
            .set_payload(RegisterAckPayload::new().set_peers(state_lock.peers.clone()))
            .build();
        state.udp.send_to(&answer, state.source.clone())
            .expect("Sending using UDP should be successful.");
    }

    if !state_lock.peers.contains(&state.source) {
        info!("Registered new peer.");
        state_lock.peers.push(state.source);
    }
}