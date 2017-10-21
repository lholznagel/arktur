use event::EventHandler;
use std::net::{IpAddr, UdpSocket, SocketAddr};
use std::str;

/// Stores all needed infomration about a udp client
pub struct UdpClient {
    /// open udp socket
    udp: UdpSocket,
    /// Handler for the register events
    handlers: EventHandler
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
            handlers: handlers
        }
    }

    /// TODO: documentation
    pub fn notify_hole_puncher(self, hole_puncher_addr: SocketAddr) -> Self {
        self.udp.send_to((String::from("REGISTER | ".to_owned() + self.udp.local_addr().unwrap().to_string().as_str())).as_bytes(), hole_puncher_addr).unwrap();
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
                    let updated_buffer = updated_buffer.as_slice();

                    let message: &str = str::from_utf8(updated_buffer).unwrap_or("");
                    let event: Vec<&str> = message.split(" |").collect();
                    println!("Message lib: {:?}", event[0]);

                    match event[0] {
                        "REGISTER" => (self.handlers.register_handler)(source, &self.udp, message),
                        "ACK_REGISTER" => (self.handlers.register_ack_handler)(source, &self.udp, message),
                        _ => {}
                    };
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
}