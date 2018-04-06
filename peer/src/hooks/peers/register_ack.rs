use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::Punsh;
use carina_protocol::payload::peers::{GetPeersAck, Register};

use hooks::State;

pub fn register_ack(state: ApplicationState<State>) {
    let message = Protocol::<GetPeersAck>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    if !message.payload.peers.is_empty() {
        for address in message.payload.peers {
            if !address.is_empty() && !state_lock.peers.contains_key(&address) {
                let payload = Punsh {
                    address: address.clone()
                };
                
                // notify hole puncher
                let result = Protocol::<Punsh>::new()
                    .set_payload(payload)
                    .set_event_code(as_number(EventCodes::Punsh))
                    .build(&state_lock.nacl);
                state.udp.send_to(&result, "0.0.0.0:50000").expect("Sending a message should be successful");

                let register = Register {
                    pub_key: state_lock.nacl.get_public_key()
                };
                let result = Protocol::<Register>::new()
                    .set_event_code(as_number(EventCodes::Register))
                    .set_payload(register)
                    .build(&state_lock.nacl);
                state.udp.send_to(&result, address.clone()).expect("Sending a response should be successful");
            }
        }
    }
}