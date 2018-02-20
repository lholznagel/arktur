use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{HolePuncherConn, RegisterAckPayload, PingPayload};

use hooks::State;

use std::{thread, time}

pub fn on_holepuncher_conn(state: ApplicationState<State>) {
    event!("HOLE_PUNCHER_CONN");
    let message = BlockchainProtocol::<HolePuncherConn>::from_bytes(&state.payload_buffer).unwrap();

    // send 4 pings with a timeout of 250 milliseconds
    for _ in 0..4 {
        let result = BlockchainProtocol::<PingPayload>::new().set_event_code(EventCodes::Ping).build();
        state.udp.send_to(&result, message.payload.address.clone()).expect("Sending a message should be successful");

        thread::sleep(time::Duration::milliseconds(250));
    }
}