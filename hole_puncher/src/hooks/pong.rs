use carina_hooks::ApplicationState;

use hooks::State;

pub fn on_pong(state: ApplicationState<State>) {
    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
    match state_lock.peers.clone().get(&state.source) {
        Some(peer) => state_lock.peers.insert(state.source, (peer.0, 0)),
        None => None
    };
}