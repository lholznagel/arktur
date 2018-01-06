extern crate blockchain_hooks;
extern crate blockchain_protocol;

use blockchain_hooks::EventCodes;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{PayloadModel, RegisterAckPayload};

#[test]
fn test_basic() {
    let blockchain_protocol = BlockchainProtocol::<RegisterAckPayload>::new();
    let expected = &[255, 255, 0, 0, 0, 0, 0, 0];
    assert_eq!(blockchain_protocol.build(), expected);
}

#[test]
fn test_set_payload() {
    let payload = RegisterAckPayload::new().set_addr(String::from("My test payload"));
    let blockchain_protocol = BlockchainProtocol::<RegisterAckPayload>::new()
      .set_payload(payload);

    let expected = &[255, 255, 0, 0, 17, 0, 34, 17, 126, 77, 121, 32, 116, 101, 115, 116, 32, 112, 97, 121, 108, 111, 97, 100, 126];
    assert_eq!(blockchain_protocol.build(), expected);
}

#[test]
fn test_set_all_values() {
    let payload = RegisterAckPayload::new().set_addr(String::from("192.168.192.2:45678"));
    let blockchain_protocol = BlockchainProtocol::<RegisterAckPayload>::new()
      .set_event_code(EventCodes::Register)
      .set_status_code(StatusCodes::Ok)
      .set_payload(payload);

    let expected = &[16, 0, 0, 0, 21, 0, 138, 37, 126, 49, 57, 50, 46, 49, 54, 56, 46, 49, 57, 50, 46, 50, 58, 52, 53, 54, 55, 56, 126];
    assert_eq!(blockchain_protocol.build(), expected);
}