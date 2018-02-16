use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{RegisterPayload, RegisterAckPayload};

use hooks::State;

pub fn on_register_peer_ack(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<RegisterAckPayload>::from_bytes(&state.payload_buffer).expect("Parsing should be successful");
    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
    event!("ACK_REGISTER {:?}", message);

    if message.status_code == StatusCodes::NoPeer {
        info!("No peer from other peer");
    } else {
        for address in message.payload.addresses {
            if !state_lock.peers.contains(&address) {
                let result = BlockchainProtocol::<RegisterPayload>::new().set_event_code(EventCodes::RegisterPeer).build();
                state.udp.send_to(&result, address.clone()).expect("Sending a response should be successful");
                state_lock.peers.push(address.clone());
                success!("Send REGISTER_PEER to {:?}", address);
            } else {
                debug!("Peer already known");
            }
        }
    }
}