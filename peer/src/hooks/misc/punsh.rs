use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::{Punsh, EmptyPayload};

use hooks::State;

use std::{thread, time};

pub fn punsh(state: ApplicationState<State>) {
    let mut nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };

    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    let message = Protocol::<Punsh>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    match state_lock.peers.get(&message.payload.address.clone()) {
        Some(contacting_peer) => {
            for _ in 0..4 {
                let result = Protocol::<EmptyPayload>::new()
                    .set_event_code(as_number(EventCodes::Ping))
                    .build(&mut nacl, &contacting_peer.0);
                state.udp.send_to(&result, message.payload.address.clone()).expect("Sending using UDP should be successful.");

                thread::sleep(time::Duration::from_millis(250));
            }
        },
        _ => ()
    };
}