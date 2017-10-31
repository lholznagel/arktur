extern crate blockchain_protocol;

use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::events::EventCodes;
use blockchain_protocol::enums::status::StatusCodes;

#[test]
fn test_basic() {
    let blockchain_protocol = BlockchainProtocol::new();
    let expected = &[255, 255, 0, 0, 0, 0, 0, 0];
    assert_eq!(blockchain_protocol.build(), expected);
}

#[test]
fn test_set_data() {
    let blockchain_protocol = BlockchainProtocol::new()
      .set_data(String::from("My testdata"));

    let expected = &[255, 255, 0, 0, 0, 0, 11, 0, 77, 121, 32, 116, 101, 115, 116, 100, 97, 116, 97];
    assert_eq!(blockchain_protocol.build(), expected);
}

#[test]
fn test_set_all_values() {
    let blockchain_protocol = BlockchainProtocol::new()
      .set_event_code(EventCodes::Register)
      .set_status_code(StatusCodes::Ok)
      .set_data(String::from("192.168.192.2:45678"));

    let expected = &[16, 0, 0, 0, 0, 0, 19, 0, 49, 57, 50, 46, 49, 54, 56, 46, 49, 57, 50, 46, 50, 58, 52, 53, 54, 55, 56];
    assert_eq!(blockchain_protocol.build(), expected);
}