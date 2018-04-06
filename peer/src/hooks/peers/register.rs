use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::Payload;
use carina_protocol::payload::peers::{RegisterAck, Register};

use hooks::State;

pub fn register(state: ApplicationState<State>) {
    let message = Protocol::<Register>::from_bytes_unencrypted(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    let peers = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.peers.clone()
    };
    let nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };

    if peers.is_empty() {
        let answer = Protocol::new()
            .set_event_code(as_number(EventCodes::RegisterAck))
            .set_payload(RegisterAck::new().set_public_key(&nacl.get_public_key()))
            .build_unencrypted();
        state.udp.send_to(&answer, state.source.clone())
            .expect("Sending using UDP should be successful.");
    } else {
        let mut known_peers = Vec::new();
        for (peer, _) in peers.clone() {
            known_peers.push(peer);
        }

        let payload = RegisterAck {
            public_key: Some(nacl.get_public_key()),
            peers: known_peers
        };
        let answer = Protocol::new()
            .set_event_code(as_number(EventCodes::RegisterAck))
            .set_payload(payload)
            .build_unencrypted();
        state.udp.send_to(&answer, state.source.clone())
            .expect("Sending using UDP should be successful.");
    }

    if !peers.contains_key(&state.source) {
        info!("Registered new peer.");
        let mut state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.peers.insert(state.source, (message.payload.pub_key, 0));
    }
}