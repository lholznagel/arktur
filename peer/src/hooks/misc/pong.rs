use blockchain_hooks::ApplicationState;

use hooks::State;

pub fn pong(state: ApplicationState<State>) {
    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
    if state_lock.peers.contains_key(&state.source) {
        state_lock.peers.insert(state.source, 0);
    }

    info!("Received PONG");
}