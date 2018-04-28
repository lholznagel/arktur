use carina_hooks::{as_number, MessageState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::EmptyPayload;

use hooks::State;

pub fn ping(state: MessageState<State>) {
    info!("[PING] Ping from {}", state.source);
    let mut nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };

    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");
    let contacting_peer = state_lock.peers.get(&state.source.clone()).unwrap();

    let answer = Protocol::<EmptyPayload>::new()
        .set_event_code(as_number(EventCodes::Pong))
        .build(&mut nacl, &contacting_peer.0);
    state.udp.send_to(&answer, state.source.clone())
        .expect("Sending using UDP should be successful.");
}