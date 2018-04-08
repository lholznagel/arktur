use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::Punsh;
use carina_protocol::payload::peers::{RegisterAck, Register};

use hooks::State;

pub fn register_ack(state: ApplicationState<State>) {
    let message = Protocol::<RegisterAck>::from_bytes_unencrypted(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    let own_public_key = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.get_public_key()
    };
    let mut nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };

    let mut state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");
    state_lock.peers.insert(state.source.clone(), (message.payload.public_key.unwrap(), 0));

    if !message.payload.peers.is_empty() {
        for address in message.payload.peers {
            if !address.is_empty() && !state_lock.peers.contains_key(&address) {
                let payload = Punsh {
                    address: address.clone()
                };
                
                // TODO: replace 127.0.0.1 with the configured peer ip
                let contacting_hole_puncher = state_lock.peers.get_mut("127.0.0.1:50000").unwrap();
                let result = Protocol::<Punsh>::new()
                    .set_payload(payload)
                    .set_event_code(as_number(EventCodes::Punsh))
                    .build(&mut nacl, &contacting_hole_puncher.0);
                state.udp.send_to(&result, "0.0.0.0:50000").expect("Sending a message should be successful");

                let register = Register {
                    public_key: own_public_key
                };
                let result = Protocol::<Register>::new()
                    .set_event_code(as_number(EventCodes::Register))
                    .set_payload(register)
                    .build_unencrypted();
                state.udp.send_to(&result, address.clone()).expect("Sending a response should be successful");
            }
        }
    }
}