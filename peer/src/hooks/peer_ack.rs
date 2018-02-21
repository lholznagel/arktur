use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{RegisterPayload, RegisterAckPayload};

use hooks::State;

pub fn on_register_peer_ack(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<RegisterAckPayload>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    let mut state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    if message.status_code == StatusCodes::NoPeer {
        info!("No peer.");
    } else {
        for address in message.payload.addresses {
            if !state_lock.peers.contains(&address) {
                let result = BlockchainProtocol::<RegisterPayload>::new().set_event_code(EventCodes::RegisterPeer).build();
                state.udp.send_to(&result, address.clone())
                    .expect("Sending using UDP should be successful.");
                info!("Registered a new peer.");
                state_lock.peers.push(address.clone());
            }
        }
    }
}