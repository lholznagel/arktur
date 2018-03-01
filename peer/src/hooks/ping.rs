use blockchain_hooks::{as_number, ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::PongPayload;

use hooks::State;

pub fn on_ping(state: ApplicationState<State>) {
    info!("Received PING. Answering with PONG.");
    let answer = BlockchainProtocol::<PongPayload>::new()
        .set_event_code(as_number(EventCodes::Pong))
        .build();
    state.udp.send_to(&answer, state.source.clone())
        .expect("Sending using UDP should be successful.");
}