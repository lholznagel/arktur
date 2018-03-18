use blockchain_hooks::{as_number, ApplicationState, EventCodes};
use blockchain_protocol::Protocol;
use blockchain_protocol::payload::EmptyPayload;

use hooks::State;

pub fn ping(state: ApplicationState<State>) {
    info!("Received PING. Answering with PONG.");
    let answer = Protocol::<EmptyPayload>::new()
        .set_event_code(as_number(EventCodes::Pong))
        .build();
    state.udp.send_to(&answer, state.source.clone())
        .expect("Sending using UDP should be successful.");
}