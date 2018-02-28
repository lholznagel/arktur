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
    pub on_ping: Option<fn(ApplicationState<T>)>,
    /// Executed on a `PONG` event
    /// Code: 1
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_pong: Option<fn(ApplicationState<T>)>,

    /// Executed on a `REGISTER_HOLE_PUNCHER` event
    /// Code: 16
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_register_hole_puncher: Option<fn(ApplicationState<T>)>,
    /// Executed on a `REGISTER_HOLE_PUNCHER_ACK` event
    /// Code: 17
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_register_hole_puncher_ack: Option<fn(ApplicationState<T>)>,
    /// Executed on a `REGISTER_PEER` event
    /// Code: 18
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_register_peer: Option<fn(ApplicationState<T>)>,
        /// Executed on a `REGISTER_PEER_ACK` event
    /// Code: 19
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_register_peer_ack: Option<fn(ApplicationState<T>)>,


    /// Executed on a `DATA_FOR_BLOCK` event
    /// Code: 32
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_data_for_block: Option<fn(ApplicationState<T>)>,
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

    /// Executed on a `SYNC_PEER` event
    /// Code: 96
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_sync_peers: Option<fn(ApplicationState<T>)>,
    /// Executed on a `SYNC_BLOCKS` event
    /// Code: 97
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_sync_blocks: Option<fn(ApplicationState<T>)>,
    /// Executed on a `SYNC_BLOCKS_ACK` event
    /// Code: 98
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_sync_blocks_ack: Option<fn(ApplicationState<T>)>,
    /// Executed on a `SYNC_BLOCKS_REQ` event
    /// Code: 99
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_sync_blocks_req: Option<fn(ApplicationState<T>)>,
    /// Executed on a `SYNC_BLOCKS_REQ_ACK` event
    /// Code: 100
    ///
    /// # Parameters
    ///
    /// - `ApplicationState` - state of the application
    pub on_sync_blocks_req_ack: Option<fn(ApplicationState<T>)>,

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
            on_ping: None,
            on_pong: None,
            on_register_hole_puncher: None,
            on_register_hole_puncher_ack: None,
            on_register_peer: None,
            on_register_peer_ack: None,
            on_data_for_block: None,
            on_new_block: None,
            on_possible_block: None,
            on_validate_hash: None,
            on_validated_hash: None,
            on_found_block: None,
            on_hole_puncher_conn: None,
            on_sync_peers: None,
            on_sync_blocks: None,
            on_sync_blocks_ack: None,
            on_sync_blocks_req: None,
            on_sync_blocks_req_ack: None,
            on_explore_network: None,
        }
    }

    /// Registers a ping hook
    pub fn set_ping(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_ping = Some(function);
        self
    }

    /// Registers a pong hook
    pub fn set_pong(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_pong = Some(function);
        self
    }

    /// Registers a register_hole_puncher hook
    pub fn set_register_hole_puncher(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_register_hole_puncher = Some(function);
        self
    }

    /// Registers a register_hole_puncher_ack hook
    pub fn set_register_hole_puncher_ack(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_register_hole_puncher_ack = Some(function);
        self
    }

    /// Registers a register_peer hook
    pub fn set_register_peer(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_register_peer = Some(function);
        self
    }

    /// Registers a register_peer_ack hook
    pub fn set_register_peer_ack(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_register_peer_ack = Some(function);
        self
    }

    /// Registers a data_for_block hook
    pub fn set_data_for_block(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_data_for_block = Some(function);
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

    /// Registers a sync_peers hook
    pub fn set_sync_peers(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_sync_peers = Some(function);
        self
    }

    /// Registers a sync_blocks hook
    pub fn set_sync_blocks(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_sync_blocks = Some(function);
        self
    }

    /// Registers a sync_blocks_ack hook
    pub fn set_sync_blocks_ack(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_sync_blocks_ack = Some(function);
        self
    }

    /// Registers a sync_blocks hook
    pub fn set_sync_blocks_req(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_sync_blocks_req = Some(function);
        self
    }

    /// Registers a sync_blocks_req_ack hook
    pub fn set_sync_blocks_req_ack(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_sync_blocks_req_ack = Some(function);
        self
    }

    /// Registers a explore_network hook
    pub fn set_explore_network(mut self, function: fn(ApplicationState<T>)) -> Self {
        self.on_explore_network = Some(function);
        self
    }
}