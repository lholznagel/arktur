use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::Protocol;
use carina_protocol::payload::peers::{RegisterAck, Register};

use hooks::State;

pub fn register(state: ApplicationState<State>) {
    info!("[REGISTER] New registration from {}", state.source);
    let message = Protocol::<Register>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    debug!("[REGISTER] message: {:?}", message);

    let peers = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.peers.clone()
    };
    let mut nacl = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.nacl.clone()
    };

    let mut payload = RegisterAck {
        public_key: Some(nacl.get_public_key()),
        peers: Vec::new()
    };

    // insert all known peer into the payload
    if !peers.is_empty() {
        for (peer, _) in peers.clone() {
            payload.peers.push(peer);
        }
    }
    // send the message
    let answer = Protocol::new()
            .set_event_code(as_number(EventCodes::RegisterAck))
            .set_payload(payload)
            .build_unencrypted(&mut nacl);
        state.udp.send_to(&answer, state.source.clone())
            .expect("Sending using UDP should be successful.");
    debug!("[REGISTER] Acknowledge registration");

    // add the new peer if it does not exist
    if !peers.contains_key(&state.source) {
        let mut state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.peers.insert(state.source, (message.payload.public_key, 0));
        info!("[REGISTER] Registered new peer.");
    }
}