use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{Payload, RegisterPayload, RegisterAckPayload};

use hooks::State;

pub fn on_register_peer(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<RegisterPayload>::from_bytes(&state.payload_buffer).unwrap();
    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
    event!("ACK_REGISTER {:?}", message);

    if state_lock.peers.is_empty() {
        sending!("ACK_REGISTER | NO_PEER");
        let answer = BlockchainProtocol::new()
            .set_event_code(EventCodes::RegisterPeerAck)
            .set_status_code(StatusCodes::NoPeer)
            .set_payload(RegisterAckPayload::new())
            .build();
        state.udp.send_to(&answer, state.source.clone()).expect("Sending a response should be successful");
    } else {
        sending!("ACK_REGISTER | PEER");
        let answer = BlockchainProtocol::new()
            .set_event_code(EventCodes::RegisterPeerAck)
            .set_status_code(StatusCodes::Ok)
            .set_payload(RegisterAckPayload::new().set_peers(state_lock.peers.clone()))
            .build();
        state.udp.send_to(&answer, state.source.clone()).expect("Sending a response should be successful");
    }

    state_lock.peers.push(state.source);
    debug!("REGISTER: {}", state_lock.peers.len());
}