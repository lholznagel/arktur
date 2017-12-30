use enums::EventCodes;
use hooks::Hooks;
use std::net::UdpSocket;

pub struct HookNotification {
    hook: Box<Hooks>
}

impl HookNotification {
    pub fn new(hook: Box<Hooks>) -> Self {
        Self {
            hook: hook
        }
    }

    /// Checks the event and executes the matching hook
    ///
    /// # Parameters
    ///
    /// - `udp` - Open UDP connection
    /// - `event` - Event code to check what event this is
    /// - `payload_buffer` - raw message
    /// - `source` - source this message comes from
    pub fn notify(&mut self, udp: &UdpSocket, event: EventCodes, payload_buffer: Vec<u8>, source: String) -> Vec<u8> {
        match event {
            EventCodes::Ping => self.hook.on_ping(payload_buffer, source),
            EventCodes::Pong => self.hook.on_pong(payload_buffer, source),
            EventCodes::Register => self.hook.on_register(udp, payload_buffer, source),
            EventCodes::AckRegister => self.hook.on_ack_register(payload_buffer, source),
            EventCodes::PeerRegistering => self.hook.on_peer_registering(payload_buffer, source),
            EventCodes::NewBlock => self.hook.on_new_block(payload_buffer, source),
            EventCodes::PossibleBlock => self.hook.on_possible_block(udp, payload_buffer, source),
            EventCodes::ValidateHash => self.hook.on_validate_hash(payload_buffer, source),
            EventCodes::ValidatedHash => self.hook.on_validated_hash(payload_buffer, source),
            EventCodes::FoundBlock => self.hook.on_found_block(payload_buffer, source),
            EventCodes::NotAValidEvent => Vec::new(),
        }
    }
}