use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{HolePuncherConn, RegisterAckPayload, PingPayload};

use hooks::State;

use std::{thread, time}

pub fn on_holepuncher_conn(state: ApplicationState<State>) {
    event!("HOLE_PUNCHER_CONN");
    let message = BlockchainProtocol::<HolePuncherConn>::from_bytes(&state.payload_buffer).unwrap();

    let payload = HolePuncherConn {
        address: state.source
    };

    let result = BlockchainProtocol::<HolePuncherConn>::new().set_payload(payload).set_event_code(EventCodes::HolePuncherConn).build();
    state.udp.send_to(&result, message.payload.address).expect("Sending a message should be successful");
}