use blockchain_file::KnownPeers;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::events::EventCodes;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{PayloadModel, PeerRegisteringPayload, RegisterPayload, RegisterAckPayload};

use std::net::{UdpSocket, SocketAddr};

/// # What does it do?
///
/// - Create a "hole" between to peers
/// - When a peer registers itself, its IP-Address + Port are saved
/// - The next peer that registers itself, gets these IP-Address + Port
/// - The older peer gets the IP-Address + Port of the new peer
/// - The address of the new peer are saved instead of the old peer
/// - Both start a ping event to the other peer
/// - With this technic a connection between two private networks can be accomplished
///
/// In the following graphic, the process is shown
///
/// ```
///  1. Register  +--------------+ 2. Register
///   +--------->|              |<---------+
///   |          | hole puncher |          |
///   |    +-----|              |-----+    |
///   |    |     +--------------+     |    |
///   |    | 3. Send IP+Port of new   |    |
///   |    |                          |    |
///   |    |                          |    |
///   |    |                          |    |
///   |    |   4. Send IP+Port of old |    |
///   |    v                          v    |
/// +--------+                      +--------+
/// |        |--------------------->|        |
/// | Peer A |      5. Contact      | Peer B |
/// |        |<---------------------|        |
/// +--------+                      +--------+
///
/// created with http://asciiflow.com/
/// ```
///
/// # Example
///
/// - Peer A runs on 192.168.1.5:45678 (on host a)
/// - Peer B runs on 192.168.1.6:56789 (on host b)
/// - Peer A registers itself at the hole puncher (some.public.ip.address:45000)
/// - The hole puncher does not know any peer
/// - Peer B registers itself at the same hole puncher
/// - The hole puncher sends the Peer B information to Peer A
/// - The hole puncher then sends the Peer A information to Peer B
/// - Peer A and Peer B try to ping each other
/// - The connection between both networks should be good to go
///
/// Handles a new peer
pub fn register_handler(source: SocketAddr, udp: &UdpSocket, register_payload: BlockchainProtocol<RegisterPayload>) {
    let last_peer = KnownPeers::get_latest();
    let mut status = StatusCodes::Ok;

    if last_peer.name == "" {
        status = StatusCodes::NoPeer;
    } else {
        let payload = PeerRegisteringPayload::new().set_addr(source.to_string());
        let message = BlockchainProtocol::new()
            .set_event_code(EventCodes::PeerRegistering)
            .set_payload(payload)
            .build();
        udp.send_to(message.as_slice(), last_peer.socket.parse::<SocketAddr>().unwrap())
            .unwrap();
    }

    KnownPeers::new(register_payload.payload.name(), source.to_string()).save();

    let payload = RegisterAckPayload::new().set_addr(last_peer.socket);
    sending!(format!("ACK_REGISTER | {:?}", payload));
    let message = BlockchainProtocol::new()
        .set_event_code(EventCodes::AckRegister)
        .set_status_code(status)
        .set_payload(payload)
        .build();
    udp.send_to(message.as_slice(), source).unwrap();
}