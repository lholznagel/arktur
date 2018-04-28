use carina_hooks::MessageState;

use hooks::State;

pub fn pong(state: MessageState<State>) {
    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
    match state_lock.peers.clone().get(&state.source) {
        Some(peer) => state_lock.peers.insert(state.source, (peer.0, 0, true)),
        None => None
    };

    info!("Received PONG");
}