use carina_hooks::{as_number, ApplicationState, EventCodes};
use carina_protocol::payload::peers::{GetPeersAck, Register};
use carina_protocol::Protocol;

use hooks::State;

pub fn get_peers_ack(state: ApplicationState<State>) {
    let message = Protocol::<GetPeersAck>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    info!("Syncing peers.");

    {
        let state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");

        for new_peer in message.payload.peers {
            if !new_peer.is_empty() && !state_lock.peers.contains_key(&new_peer) {
                let register = Register {
                    pub_key: state_lock.nacl.get_public_key()
                };
                let result = Protocol::<Register>::new()
                    .set_event_code(as_number(EventCodes::Register))
                    .set_payload(register)
                    .build(&state_lock.nacl);
                state.udp.send_to(&result, new_peer).expect("Sending a response should be successful");
            }
        }
    }
}