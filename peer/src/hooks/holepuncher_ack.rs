use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{HolePuncherConn, RegisterPayload, RegisterAckPayload};

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
            let payload = HolePuncherConn {
                address: address.clone()
            };

            // notify hole puncher
            let result = BlockchainProtocol::<HolePuncherConn>::new().set_payload(payload).set_event_code(EventCodes::HolePuncherConn).build();
            state.udp.send_to(&result, "0.0.0.0:50000").expect("Sending a message should be successful");

            // try to register 4 times with a sleep of 4 ms
            for _ in 0..4 {
                let result = BlockchainProtocol::<RegisterPayload>::new().set_event_code(EventCodes::RegisterPeer).build();
                state.udp.send_to(&result, address.clone()).expect("Sending a response should be successful");
                success!("Send REGISTER_PEER to {:?}", address);
            }

            if !state_lock.peers.contains(&address) {
                state_lock.peers.push(address);
            }
        }
    }
}