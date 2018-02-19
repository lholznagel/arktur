use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::payload::{Payload, ExploreNetworkPayload};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;

use hooks::State;

pub fn on_explore_network(state: ApplicationState<State>) {
    debug!("Sending peers to debugger");
    let state_lock = state.state.lock().expect("Locking the mutex should be successful.");

    let answer = BlockchainProtocol::new()
        .set_event_code(EventCodes::ExploreNetwork)
        .set_status_code(StatusCodes::Ok)
        .set_payload(ExploreNetworkPayload::new().set_peers(state_lock.peers.clone()))
        .build();
    state.udp.send_to(&answer, state.source).expect("Sending a response should be successful");
}