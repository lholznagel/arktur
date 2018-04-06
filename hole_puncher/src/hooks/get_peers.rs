use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::Payload;
use carina_protocol::payload::peers::GetPeersAck;

use hooks::State;

pub fn get_peers(state: ApplicationState<State>) {
    let mut nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };

    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");
    let contacting_peer = state_lock.peers.get(&state.source).unwrap();

    let mut peers = Vec::new();
    for (peer, _) in state_lock.peers.clone() {
        peers.push(peer);
    }

    let answer = Protocol::new()
        .set_event_code(as_number(EventCodes::GetPeersAck))
        .set_payload(GetPeersAck::new().set_peers(peers))
        .build(&mut nacl, &contacting_peer.0);

    info!("Sending Debugger all peers.");
    state.udp.send_to(&answer, state.source)
        .expect("Sending using UDP should be successful.");
}