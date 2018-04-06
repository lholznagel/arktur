use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::payload::peers::{GetPeersAck, Register};
use carina_protocol::Protocol;

use hooks::State;

pub fn get_peers_ack(state: ApplicationState<State>) {
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
    let peers = {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");
        state_lock.peers.clone()
    };

    let mut state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");
    let message = Protocol::<GetPeersAck>::from_bytes(&state.payload_buffer, &nacl, &source_peer.0)
        .expect("Parsing the protocol should be successful.");
    info!("Syncing peers.");

    {
        for new_peer in message.payload.peers {
            let contacting_peer = state_lock.peers.get_mut(&new_peer).unwrap();
            if !new_peer.is_empty() && !peers.contains_key(&new_peer) {
                let register = Register {
                    pub_key: nacl.get_public_key()
                };
                let result = Protocol::<Register>::new()
                    .set_event_code(as_number(EventCodes::Register))
                    .set_payload(register)
                    .build(&mut nacl, &contacting_peer.0);
                state.udp.send_to(&result, new_peer).expect("Sending a response should be successful");
            }
        }
    }
}