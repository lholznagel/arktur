use state::ApplicationState;

/// Struct for registering all available hooks
#[derive(Debug)]
pub struct Hooks<T> {
    /// Executed on a `PING` event
    /// Code: 0
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub ping: Option<fn(ApplicationState<T>)>,
    /// Executed on a `PONG` event
    /// Code: 1
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub pong: Option<fn(ApplicationState<T>)>,

    /// Executed on a `GET_PEERS` event
    /// Code: 64
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub get_peers: Option<fn(ApplicationState<T>)>,
    /// Executed on a `GET_PEERS_ACK` event
    /// Code: 65
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub get_peers_ack: Option<fn(ApplicationState<T>)>,
    /// Executed on a `REGISTER` event
    /// Code: 66
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub register: Option<fn(ApplicationState<T>)>,
    /// Executed on a `REGISTER_ACK` event
    /// Code: 67
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub register_ack: Option<fn(ApplicationState<T>)>,
    /// Executed on a `GET_BLOCKS` event
    /// Code: 128
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub get_blocks: Option<fn(ApplicationState<T>)>,
    /// Executed on a `GET_BLOCKS_ACK` event
    /// Code: 129
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub get_blocks_ack: Option<fn(ApplicationState<T>)>,
    /// Executed on a `GET_BLOCK` event
    /// Code: 130
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub get_block: Option<fn(ApplicationState<T>)>,
    /// Executed on a `GET_BLOCK_ACK` event
    /// Code: 131
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub get_block_ack: Option<fn(ApplicationState<T>)>,
    /// Executed on a `BLOCK_DATA` event
    /// Code: 132
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub block_data: Option<fn(ApplicationState<T>)>,

    /// Executed on a `NEW_BLOCK` event
    /// Code: 33
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_new_block: Option<fn(ApplicationState<T>)>,
    /// Executed on a `POSSIBLE_BLOCK` event
    /// Code: 34
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_possible_block: Option<fn(ApplicationState<T>)>,
    /// Executed on a `VALIDATE_HASH` event
    /// Code: 35
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_validate_hash: Option<fn(ApplicationState<T>)>,
    /// Executed on a `VALIDATED_HASH` event
    /// Code: 36
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_validated_hash: Option<fn(ApplicationState<T>)>,
    /// Executed on a `FOUND_BLOCK` event
    /// Code: 37
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_found_block: Option<fn(ApplicationState<T>)>,

    /// Executed on a `HOLE_PUNCHER_CONN` event
    /// Code: 48
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_hole_puncher_conn: Option<fn(ApplicationState<T>)>,

    /// Executed on a `EXPLORE_NETWORK` event
    /// Code: 240
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_explore_network: Option<fn(ApplicationState<T>)>,
}

impl<T> Hooks<T> {
    /// Creates a new hok register
    pub fn new() -> Self {
        Self {
            ping: None,
            pong: None,
            get_peers: None,
            get_peers_ack: None,
            register: None,
            register_ack: None,
            get_blocks: None,
            get_blocks_ack: None,
            get_block: None,
            get_block_ack: None,
            block_data: None,

            on_new_block: None,
            on_possible_block: None,
            on_validate_hash: None,
            on_validated_hash: None,
            on_found_block: None,
            on_hole_puncher_conn: None,
            on_explore_network: None,
        }
    }

    /// Registers a ping hook
    pub fn set_ping(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.ping = Some(function);
        self
    }

    /// Registers a pong hook
    pub fn set_pong(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.pong = Some(function);
        self
    }

    /// Registers a get_peers hook
    pub fn set_get_peers(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.get_peers = Some(function);
        self
    }

    /// Registers a get_peers_ack hook
    pub fn set_get_peers_ack(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.get_peers = Some(function);
        self
    }

    /// Registers a register hook
    pub fn set_register(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.register = Some(function);
        self
    }

    /// Registers a register_ack hook
    pub fn set_register_ack(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.register_ack = Some(function);
        self
    }

    /// Registers a get_blocks hook
    pub fn set_get_blocks(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.get_blocks = Some(function);
        self
    }

    /// Registers a get_blocks_ack hook
    pub fn set_get_blocks_ack(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.get_blocks_ack = Some(function);
        self
    }

    /// Registers a get_block hook
    pub fn set_get_block(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.get_block = Some(function);
        self
    }

    /// Registers a get_block_ack hook
    pub fn set_get_block_ack(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.get_block_ack = Some(function);
        self
    }

    /// Registers a block_data hook
    pub fn set_block_data(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.block_data = Some(function);
        self
    }

    /// Registers a new_block hook
    pub fn set_new_block(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_new_block = Some(function);
        self
    }

    /// Registers a possible_block hook
    pub fn set_possible_block(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_possible_block = Some(function);
        self
    }

    /// Registers a validate_hash hook
    pub fn set_validate_hash(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_validate_hash = Some(function);
        self
    }

    /// Registers a validated_hash hook
    pub fn set_validated_hash(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_validated_hash = Some(function);
        self
    }

    /// Registers a found_block hook
    pub fn set_found_block(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_found_block = Some(function);
        self
    }

    /// Registers a hole_puncher_conn hook
    pub fn set_hole_puncher_conn(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_hole_puncher_conn = Some(function);
        self
    }

    /// Registers a explore_network hook
    pub fn set_explore_network(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_explore_network = Some(function);
        self
    }
}