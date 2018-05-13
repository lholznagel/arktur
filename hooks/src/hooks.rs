use message_state::MessageState;
use hook_codes::HookCodes;

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

    /// Adds a new function to a hook
    pub fn add(mut self, hook: HookCodes, function: fn(MessageState<T>)) -> Self {
        match hook {
            HookCodes::Ping => self.ping = Some(function),
            HookCodes::Pong => self.pong = Some(function),
            HookCodes::Register => self.register = Some(function),
            HookCodes::RegisterAck => self.register_ack = Some(function),
            HookCodes::GetBlocks => self.get_blocks = Some(function),
            HookCodes::GetBlocksAck => self.get_blocks_ack = Some(function),
            HookCodes::GetBlock => self.get_block = Some(function),
            HookCodes::GetBlockAck => self.get_block_ack = Some(function),
            HookCodes::BlockData => self.block_data = Some(function),
            HookCodes::BlockGen => self.block_gen = Some(function),
            HookCodes::BlockFound => self.block_found = Some(function),
            HookCodes::HashVal => self.hash_val = Some(function),
            HookCodes::HashValAck => self.hash_val_ack = Some(function),
            HookCodes::NotAValidType => ()
        };
        self
    }
}