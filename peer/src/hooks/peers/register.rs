use blockchain_hooks::{as_number, ApplicationState, EventCodes};
use blockchain_protocol::Protocol;
use blockchain_protocol::payload::Payload;
use blockchain_protocol::payload::peers::GetPeersAck;

use hooks::State;

pub fn register(state: ApplicationState<State>) {
    let mut state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    if state_lock.peers.is_empty() {
        let answer = Protocol::new()
            .set_event_code(as_number(EventCodes::RegisterAck))
            .set_payload(GetPeersAck::new())
            .build();
        state.udp.send_to(&answer, state.source.clone())
            .expect("Sending using UDP should be successful.");
    } else {
        let mut peers = Vec::new();
        for (peer, _) in state_lock.peers.clone() {
            peers.push(peer);
        }

        let answer = Protocol::new()
            .set_event_code(as_number(EventCodes::RegisterAck))
            .set_payload(GetPeersAck::new().set_peers(peers))
            .build();
        state.udp.send_to(&answer, state.source.clone())
            .expect("Sending using UDP should be successful.");
    }

    if !state_lock.peers.contains_key(&state.source) {
        info!("Registered new peer.");
        state_lock.peers.insert(state.source, 0);
    }
}