use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::Payload;
use carina_protocol::payload::peers::GetPeersAck;

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
        .set_payload(GetPeersAck::new().set_peers(peers))
        .build(&state_lock.nacl);

    info!("Sending Debugger all peers.");
    state.udp.send_to(&answer, state.source)
        .expect("Sending using UDP should be successful.");
}