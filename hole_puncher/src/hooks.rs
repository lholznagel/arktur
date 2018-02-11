use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::payload::{Payload, RegisterAckPayload, ExploreNetworkPayload};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;

pub struct StateHandler {
    peers: Vec<String>
}

impl StateHandler {
    pub fn new() -> Self {
        Self {
            peers: Vec::new()
        }
    }
}

pub fn on_register_hole_puncher(state: ApplicationState<StateHandler>) {
    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
    event!("New peer: {}", state.source);
    if state_lock.peers.is_empty() {
        sending!("ACK_REGISTER | NO_PEER");
        let answer = BlockchainProtocol::new()
            .set_event_code(EventCodes::RegisterHolePuncherAck)
            .set_status_code(StatusCodes::NoPeer)
            .set_payload(RegisterAckPayload::new())
            .build();
        state.udp.send_to(&answer, state.source.clone()).expect("Sending a response should be successful");
    } else {
        sending!("ACK_REGISTER | PEER");
        let answer = BlockchainProtocol::new()
            .set_event_code(EventCodes::RegisterHolePuncherAck)
            .set_status_code(StatusCodes::Ok)
            .set_payload(RegisterAckPayload::new().set_peers(state_lock.peers.clone()))
            .build();
        state.udp.send_to(&answer, state.source.clone()).expect("Sending a response should be successful");
    }

    state_lock.peers.push(state.source);
}

pub fn on_explore_network(state: ApplicationState<StateHandler>) {
    debug!("Sending peers to debugger");
    let state_lock = state.state.lock().expect("Locking the mutex should be successful.");

    let answer = BlockchainProtocol::new()
        .set_event_code(EventCodes::ExploreNetwork)
        .set_status_code(StatusCodes::Ok)
        .set_payload(ExploreNetworkPayload::new().set_peers(state_lock.peers.clone()))
        .build();
    state.udp.send_to(&answer, state.source).expect("Sending a response should be successful");
}