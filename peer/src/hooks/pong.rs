use blockchain_hooks::ApplicationState;

use hooks::State;

pub fn on_pong(_: ApplicationState<State>) {
    info!("Received PONG");
}