use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::PongPayload;

use hooks::State;

pub fn on_ping(state: ApplicationState<State>) {
    event!("PING from peer {:?}", state.source);
    sending!("PONG to peer {:?}", state.source);
    let answer = BlockchainProtocol::<PongPayload>::new().set_event_code(EventCodes::Pong).build();
    state.udp.send_to(&answer, state.source.clone()).expect("Sending a response should be successful");
    success!("Send PONG to {:?}", state.source);
}