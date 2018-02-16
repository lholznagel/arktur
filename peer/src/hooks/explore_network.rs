use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::{ExploreNetworkPayload, Payload};

use hooks::State;

pub fn on_explore_network(state: ApplicationState<State>) {
    debug!("Sending peers to debugger");
    let state_lock = state.state.lock().expect("Locking should be successful");

    let answer = BlockchainProtocol::new()
        .set_event_code(EventCodes::ExploreNetwork)
        .set_payload(ExploreNetworkPayload::new().set_peers(state_lock.peers.clone()))
        .build();
    state.udp.send_to(&answer, state.source).expect("Sending a response should be successful");
}