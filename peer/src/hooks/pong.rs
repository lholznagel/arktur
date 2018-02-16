use blockchain_hooks::ApplicationState;

use hooks::State;

pub fn on_pong(state: ApplicationState<State>) {
    event!("PONG from peer {:?}", state.source);
}