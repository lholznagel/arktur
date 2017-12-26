use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::events::{as_enum, EventCodes};
use blockchain_protocol::payload::*;
use event::{EventHandler, EventRegister};
use std::net::{IpAddr, UdpSocket, SocketAddr};

/// Stores all needed information about a udp client
pub struct UdpClient {
    /// open udp socket
    udp: UdpSocket,
    /// Handler for the register events
    handlers: EventHandler,
    /// Contains all registered event handlers
    register: EventRegister
}

impl UdpClient {
    /// Creates a new UdpClient
    ///
    /// # Returns
    ///
    /// New instance of `UdpClient`
    pub fn new(udp: UdpSocket, handlers: EventHandler, register: EventRegister) -> Self {
        UdpClient {
            udp: udp,
            handlers: handlers,
            register: register
        }
    }

    /// Notifies the hole puncher one th given address
    ///
    /// # Parameter
    ///
    /// `address` - Address of the hole puncher server
    ///
    /// # Return
    ///
    /// Instance of the `UdpClient`
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::event::{EventHandler, EventRegister};
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    ///
    /// let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// UdpClientBuilder::new().set_port(50000).build(EventHandler::new(), EventRegister::new()).notify_hole_puncher(address, String::from("PeerName"));
    /// ```
    pub fn notify_hole_puncher(self, address: SocketAddr, name: String) -> Self {
        let payload = RegisterPayload::new().set_name(name);

        let message = BlockchainProtocol::<RegisterPayload>::new()
            .set_event_code(EventCodes::Register)
            .set_payload(payload)
            .build();

        self.udp.send_to(message.as_slice(), address).unwrap();
        self
    }

    /// Gets the open socket connection
    ///
    /// # Return
    ///
    /// `UdpSocket` - Open UdpSocket
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use blockchain_network::event::{EventHandler, EventRegister};
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client = udp_client_builder.build(EventHandler::new(), EventRegister::new());
    ///
    /// let data = [0; 10];
    /// let address = "0.0.0.0:50000";
    /// udp_client.connection().send_to(&data, address).unwrap();
    /// ```
    pub fn connection(&self) -> UdpSocket {
        self.udp.try_clone().unwrap()
    }

    /// Gets the current port
    ///
    /// # Return
    ///
    /// `u16` - Port the socket runs on
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use blockchain_network::event::{EventHandler, EventRegister};
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client = udp_client_builder.build(EventHandler::new(), EventRegister::new());
    ///
    /// println!("Port: {:?}", udp_client.port());
    /// ```
    pub fn port(self) -> u16 {
        self.udp.local_addr().unwrap().port()
    }

    /// Gets the current IP-Address
    ///
    /// # Return
    ///
    /// `IpAddr` - IP-Address the socket runs on
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use blockchain_network::event::{EventHandler, EventRegister};
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client = udp_client_builder.build(EventHandler::new(), EventRegister::new());
    ///
    /// println!("IP-Address: {:?}", udp_client.ip());
    /// ```
    pub fn ip(self) -> IpAddr {
        self.udp.local_addr().unwrap().ip()
    }

    /// Listens to new UDP packages
    ///
    /// When a new event is identified the given callback is called
    ///
    /// This function is blocking!
    pub fn listen(self) -> Self {
        loop {
            let mut buffer = [0; 1024];

            match self.udp.recv_from(&mut buffer) {
                Ok((bytes, source)) => {
                    let mut updated_buffer = Vec::new();
                    for i in 0..bytes {
                        updated_buffer.push(buffer[i])
                    }

                    match as_enum(updated_buffer[0]) {
                        EventCodes::Ping => {
                            let data = BlockchainProtocol::<PingPayload>::from_vec(updated_buffer);
                            let result = self.register.ping_handler.clone().handle_event(data, source);
                            &self.udp.send_to(result.as_slice(), source).unwrap();
                            
                        }
                        EventCodes::Pong => {
                            let data = BlockchainProtocol::<PongPayload>::from_vec(updated_buffer);
                            (self.handlers.pong_handler)(source, &self.udp, data);
                        }
                        EventCodes::Register => {
                            let data = BlockchainProtocol::<RegisterPayload>::from_vec(updated_buffer);
                            (self.handlers.register_handler)(source, &self.udp, data);
                        }
                        EventCodes::AckRegister => {
                            let data = BlockchainProtocol::<RegisterAckPayload>::from_vec(updated_buffer);
                            (self.handlers.register_ack_handler)(source, &self.udp, data);
                        }
                        EventCodes::PeerRegistering => {
                            let data = BlockchainProtocol::<PeerRegisteringPayload>::from_vec(updated_buffer);
                            (self.handlers.peer_registering_handler)(source, &self.udp, data);
                        }
                        EventCodes::NewBlock => {
                            let data = BlockchainProtocol::<NewBlockPayload>::from_vec(updated_buffer);
                            (self.handlers.new_block_handler)(source, &self.udp, data);
                        }
                        EventCodes::PossibleBlock => {
                            let data = BlockchainProtocol::<PossibleBlockPayload>::from_vec(updated_buffer);
                            (self.handlers.possible_block_handler)(source, &self.udp, data);
                        }
                        EventCodes::FoundBlock => {
                            let data = BlockchainProtocol::<FoundBlockPayload>::from_vec(updated_buffer);
                            (self.handlers.found_block_handler)(source, &self.udp, data);
                        }
                        EventCodes::NotAValidEvent => {}
                    };
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
}