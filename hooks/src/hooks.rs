use message_state::MessageState;

/// Struct for registering all available hooks
#[derive(Clone, Debug)]
pub struct Hooks<T> {
    /// Executed on a `PING` event
    /// Code: 0
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub ping: Option<fn(MessageState<T>)>,
    /// Executed on a `PONG` event
    /// Code: 1
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub pong: Option<fn(MessageState<T>)>,

    /// Executed on a `REGISTER` event
    /// Code: 66
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub register: Option<fn(MessageState<T>)>,
    /// Executed on a `REGISTER_ACK` event
    /// Code: 67
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub register_ack: Option<fn(MessageState<T>)>,
    /// Executed on a `GET_BLOCKS` event
    /// Code: 128
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub get_blocks: Option<fn(MessageState<T>)>,
    /// Executed on a `GET_BLOCKS_ACK` event
    /// Code: 129
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub get_blocks_ack: Option<fn(MessageState<T>)>,
    /// Executed on a `GET_BLOCK` event
    /// Code: 130
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub get_block: Option<fn(MessageState<T>)>,
    /// Executed on a `GET_BLOCK_ACK` event
    /// Code: 131
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub get_block_ack: Option<fn(MessageState<T>)>,
    /// Executed on a `BLOCK_DATA` event
    /// Code: 132
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub block_data: Option<fn(MessageState<T>)>,
    /// Executed on a `BLOCK_GEN` event
    /// Code: 133
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub block_gen: Option<fn(MessageState<T>)>,
    /// Executed on a `BLOCK_FOUND` event
    /// Code: 134
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub block_found: Option<fn(MessageState<T>)>,
    /// Executed on a `HASH_VAL` event
    /// Code: 135
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub hash_val: Option<fn(MessageState<T>)>,
    /// Executed on a `HASH_VAL_ACK` event
    /// Code: 136
    ///
    /// # Parameters
    ///
    /// - `MessageState` - state of the application
    pub hash_val_ack: Option<fn(MessageState<T>)>
}

impl<T> Hooks<T> {
    /// Creates a new hok register
    pub fn new() -> Self {
        Self {
            ping: None,
            pong: None,
            register: None,
            register_ack: None,
            get_blocks: None,
            get_blocks_ack: None,
            get_block: None,
            get_block_ack: None,
            block_data: None,
            block_gen: None,
            block_found: None,
            hash_val: None,
            hash_val_ack: None
        }
    }

    /// Registers a ping hook
    pub fn set_ping(mut self, function: fn(MessageState<T>)) -> Self {
        self.ping = Some(function);
        self
    }

    /// Registers a pong hook
    pub fn set_pong(mut self, function: fn(MessageState<T>)) -> Self {
        self.pong = Some(function);
        self
    }

    /// Registers a register hook
    pub fn set_register(mut self, function: fn(MessageState<T>)) -> Self {
        self.register = Some(function);
        self
    }

    /// Registers a register_ack hook
    pub fn set_register_ack(mut self, function: fn(MessageState<T>)) -> Self {
        self.register_ack = Some(function);
        self
    }

    /// Registers a get_blocks hook
    pub fn set_get_blocks(mut self, function: fn(MessageState<T>)) -> Self {
        self.get_blocks = Some(function);
        self
    }

    /// Registers a get_blocks_ack hook
    pub fn set_get_blocks_ack(mut self, function: fn(MessageState<T>)) -> Self {
        self.get_blocks_ack = Some(function);
        self
    }

    /// Registers a get_block hook
    pub fn set_get_block(mut self, function: fn(MessageState<T>)) -> Self {
        self.get_block = Some(function);
        self
    }

    /// Registers a get_block_ack hook
    pub fn set_get_block_ack(mut self, function: fn(MessageState<T>)) -> Self {
        self.get_block_ack = Some(function);
        self
    }

    /// Registers a block_data hook
    pub fn set_block_data(mut self, function: fn(MessageState<T>)) -> Self {
        self.block_data = Some(function);
        self
    }

    /// Registers a block_gen hook
    pub fn set_block_gen(mut self, function: fn(MessageState<T>)) -> Self {
        self.block_gen = Some(function);
        self
    }

    /// Registers a block_found hook
    pub fn set_block_found(mut self, function: fn(MessageState<T>)) -> Self {
        self.block_found = Some(function);
        self
    }

    /// Registers a hash_val hook
    pub fn set_hash_val(mut self, function: fn(MessageState<T>)) -> Self {
        self.hash_val = Some(function);
        self
    }

    /// Registers a hash_val_ack hook
    pub fn set_hash_val_ack(mut self, function: fn(MessageState<T>)) -> Self {
        self.hash_val_ack = Some(function);
        self
    }
}