use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::peers::{RegisterAck, Register};

use hooks::State;

pub fn register(state: ApplicationState<State>) {
    info!("New peer registering.");
    let nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };

    let message = Protocol::<Register>::from_bytes_unencrypted(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    let mut state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");
    if state_lock.peers.is_empty() {
        let payload = RegisterAck {
            public_key: Some(nacl.get_public_key()),
            peers: Vec::new()
        };
        let answer = Protocol::new()
            .set_event_code(as_number(EventCodes::RegisterAck))
            .set_payload(payload)
            .build_unencrypted();
        state.udp.send_to(&answer, state.source.clone())
            .expect("Sending using UDP should be successful.");
    } else {
        info!("Got some peers.");
        let mut peers = Vec::new();
        for (peer, _) in state_lock.peers.clone() {
            peers.push(peer);
        }

        let payload = RegisterAck {
            public_key: Some(nacl.get_public_key()),
            peers: peers
        };
        let answer = Protocol::new()
            .set_event_code(as_number(EventCodes::RegisterAck))
            .set_payload(payload)
            .build_unencrypted();
        state.udp.send_to(&answer, state.source.clone())
            .expect("Sending using UDP should be successful.");
    }

    state_lock.peers.insert(state.source, (message.payload.public_key, 0));
}