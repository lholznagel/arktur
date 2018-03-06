use blockchain_hooks::{as_number, ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::{RegisterPayload, HolePuncherConn, RegisterAckPayload};

use hooks::State;

pub fn register_ack(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<RegisterAckPayload>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    let mut state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    if !message.payload.addresses.is_empty() {
        for address in message.payload.addresses {
            if !state_lock.peers.contains_key(&address) {
                let payload = HolePuncherConn {
                    address: address.clone()
                };
                // notify hole puncher
                let result = BlockchainProtocol::<HolePuncherConn>::new()
                    .set_payload(payload)
                    .set_event_code(as_number(EventCodes::HolePuncherConn))
                    .build();
                state.udp.send_to(&result, "0.0.0.0:50000").expect("Sending a message should be successful");

                let result = BlockchainProtocol::<RegisterPayload>::new()
                    .set_event_code(as_number(EventCodes::Register))
                    .build();
                state.udp.send_to(&result, address.clone()).expect("Sending a response should be successful");

                if !state_lock.peers.contains_key(&address) {
                    info!("Registered new peer.");
                    state_lock.peers.insert(address, 0);
                }
            }
        }
    }
}