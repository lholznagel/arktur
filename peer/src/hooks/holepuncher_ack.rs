use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{RegisterPayload, RegisterAckPayload};

use hooks::State;

pub fn on_register_hole_puncher_ack(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<RegisterAckPayload>::from_bytes(&state.payload_buffer).unwrap();
    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
    event!("ACK_REGISTER {:?}", message);

    if message.status_code == StatusCodes::NoPeer {
        info!("No peer registered at the hole puncher");
    } else {
        sending!("REGISTER to peer {:?}", message.payload);

        for address in message.payload.addresses {
            let result = BlockchainProtocol::<RegisterPayload>::new().set_event_code(EventCodes::RegisterPeer).build();
            state.udp.send_to(&result, address.clone()).expect("Sending a response should be successful");
            success!("Send REGISTER_PEER to {:?}", address);

            if !state_lock.peers.contains(&address) {
                state_lock.peers.push(address);
            }
        }
    }
}