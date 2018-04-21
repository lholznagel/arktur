use carina_hooks::ApplicationState;
use carina_protocol::Protocol;
use carina_protocol::payload::peers::RegisterAck;

use hooks::State;

pub fn register_ack(state: ApplicationState<State>) {
    info!("[REGISTER_ACK] Registration acknowledge from {}", state.source);
    let message = Protocol::<RegisterAck>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    debug!("[REGISTER_ACK] message: {:?}", message);
}