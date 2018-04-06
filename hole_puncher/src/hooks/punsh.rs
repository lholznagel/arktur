use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::Punsh;

use hooks::State;

pub fn punsh(state: ApplicationState<State>) {
    let mut nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };
    let source_peer = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.peers.get(&state.source.clone()).unwrap().clone()
    };

    let message = Protocol::<Punsh>::from_bytes(&state.payload_buffer, &nacl, &source_peer.0)
        .expect("Parsing the protocol should be successful.");

    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");
    let contacting_peer = state_lock.peers.get(&message.payload.address).unwrap();

    if !message.payload.address.is_empty() {
        let payload = Punsh {
            address: state.source
        };

        let result = Protocol::<Punsh>::new()
            .set_payload(payload)
            .set_event_code(as_number(EventCodes::Punsh))
            .build(&mut nacl, &contacting_peer.0);

        state.udp.send_to(&result, message.payload.address)
            .expect("Sending using UDP should be successful.");
    }
}