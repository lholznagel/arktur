use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::{Punsh, EmptyPayload};

use hooks::State;

use std::{thread, time};

pub fn punsh(state: ApplicationState<State>) {
    let message = Protocol::<Punsh>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    info!("Sending pings to new peer.");
    // send 4 pings with a timeout of 250 milliseconds
    for _ in 0..4 {
        let result = Protocol::<EmptyPayload>::new().set_event_code(as_number(EventCodes::Ping)).build();
        state.udp.send_to(&result, message.payload.address.clone()).expect("Sending using UDP should be successful.");

        thread::sleep(time::Duration::from_millis(250));
    }
}