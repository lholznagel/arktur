use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::events::EventCodes;
use event::EventHandler;
use std::net::{IpAddr, UdpSocket, SocketAddr};

/// Stores all needed infomration about a udp client
pub struct UdpClient {
    /// open udp socket
    udp: UdpSocket,
    /// Handler for the register events
    handlers: EventHandler,
}

impl UdpClient {
    /// Creates a new UdpClient
    ///
    /// # Returns
    ///
    /// New instance of `UdpClient`
    pub fn new(udp: UdpSocket, handlers: EventHandler) -> Self {
        UdpClient {
            udp: udp,
            handlers: handlers,
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
    /// use blockchain_network::event::EventHandler;
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    ///
    /// let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// UdpClientBuilder::new().set_port(50000).build(EventHandler::new()).notify_hole_puncher(address);
    /// ```
    pub fn notify_hole_puncher(self, address: SocketAddr) -> Self {
        let message = BlockchainProtocol::new()
            .set_event_code(EventCodes::Register)
            .set_data(String::from(
                self.udp.local_addr().unwrap().to_string().as_str(),
            ))
            .build();

        self.udp.send_to(message.as_slice(), address).unwrap();
        self
    }

    /// Gets the open socket connection
    ///
    /// # Return
    ///
    /// `UdpSocket` - Open UdpSocet
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use blockchain_network::event::EventHandler;
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client = udp_client_builder.build(EventHandler::new());
    ///
    /// let data = [0; 10];
    /// let address = "0.0.0.0:50000";
    /// udp_client.connection().send_to(&data, address);
    /// ```
    pub fn connection(self) -> UdpSocket {
        self.udp
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
    /// use blockchain_network::event::EventHandler;
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client = udp_client_builder.build(EventHandler::new());
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
    /// use blockchain_network::event::EventHandler;
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client = udp_client_builder.build(EventHandler::new());
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
        //self.notify_hole_puncher();

        loop {
            let mut buffer = [0; 1024];

            match self.udp.recv_from(&mut buffer) {
                Ok((bytes, source)) => {
                    let mut updated_buffer = Vec::new();
                    for i in 0..bytes {
                        updated_buffer.push(buffer[i])
                    }
                    let protocol = BlockchainProtocol::from_vec(updated_buffer);

                    match protocol.event_code {
                        EventCodes::Ping => {
                            (self.handlers.ping_handler)(source, &self.udp, protocol)
                        }
                        EventCodes::Pong => {
                            (self.handlers.pong_handler)(source, &self.udp, protocol)
                        }
                        EventCodes::Register => {
                            (self.handlers.register_handler)(source, &self.udp, protocol)
                        }
                        EventCodes::AckRegister => {
                            (self.handlers.register_ack_handler)(source, &self.udp, protocol)
                        }
                        EventCodes::PeerRegistering => {
                            (self.handlers.peer_registering_handler)(source, &self.udp, protocol)
                        }
                        EventCodes::NotAValidEvent => {}
                    };
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
}