use blockchain_hooks::{as_number, ApplicationState, EventCodes};
use blockchain_protocol::Protocol;
use blockchain_protocol::payload::Payload;
use blockchain_protocol::payload::peers::GetPeersAckPayload;

use hooks::State;

pub fn get_peers(state: ApplicationState<State>) {
    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    let mut peers = Vec::new();
    for (peer, _) in state_lock.peers.clone() {
        peers.push(peer);
    }

    let answer = Protocol::new()
        .set_event_code(as_number(EventCodes::GetPeersAck))
        .set_payload(GetPeersAckPayload::new().set_peers(peers))
        .build();
    state.udp.send_to(&answer, state.source.clone())
        .expect("Sending using UDP should be successful.");
}