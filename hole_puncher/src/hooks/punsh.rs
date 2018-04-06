use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::Punsh;

use hooks::State;

pub fn punsh(state: ApplicationState<State>) {
    let message = Protocol::<Punsh>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    if !message.payload.address.is_empty() {
        let payload = Punsh {
            address: state.source
        };

        let result = Protocol::<Punsh>::new()
            .set_payload(payload)
            .set_event_code(as_number(EventCodes::Punsh))
            .build(&state_lock.nacl);

        state.udp.send_to(&result, message.payload.address)
            .expect("Sending using UDP should be successful.");
    }
}