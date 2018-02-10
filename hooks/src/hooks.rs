use state::ApplicationState;

/// Trait containing all available hooks, clients can listen to
///
/// # Example how to implement. (Taken from `empty.rs`)
///
/// ```
/// use blockchain_hooks::Hooks;
/// use blockchain_hooks::ApplicationState;
/// 
/// pub struct State;
///
/// pub struct Empty;
/// 
/// impl Hooks<State> for Empty {
///     fn on_ping(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_pong(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_register_hole_puncher(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_register_hole_puncher_ack(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_register_peer(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_register_peer_ack(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_data_for_block(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_new_block(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_possible_block(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_validate_hash(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_validated_hash(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_found_block(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
///
///     fn on_explore_network(&self, state: ApplicationState<State>) {
///         // handle hook
///     }
/// }
/// ```
pub trait Hooks<T> {
    /// Executed on a `PING` event
    /// Code: 0
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_ping(&self, state: ApplicationState<T>);

    /// Executed on a `PONG` event
    /// Code: 1
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_pong(&self, state: ApplicationState<T>);

    /// Executed on a `REGISTER_HOLE_PUNCHER` event
    /// Code: 16
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_register_hole_puncher(&self, state: ApplicationState<T>);

    /// Executed on a `REGISTER_HOLE_PUNCHER_ACK` event
    /// Code: 17
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_register_hole_puncher_ack(&self, state: ApplicationState<T>);

    /// Executed on a `REGISTER_PEER` event
    /// Code: 18
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_register_peer(&self, state: ApplicationState<T>);

    /// Executed on a `REGISTER_PEER_ACK` event
    /// Code: 19
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_register_peer_ack(&self, state: ApplicationState<T>);

    /// Executed on a `DATA_FOR_BLOCK` event
    /// Code: 32
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_data_for_block(&self, state: ApplicationState<T>);

    /// Executed on a `NEW_BLOCK` event
    /// Code: 33
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_new_block(&self, state: ApplicationState<T>);

    /// Executed on a `POSSIBLE_BLOCK` event
    /// Code: 34
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_possible_block(&self, state: ApplicationState<T>);

    /// Executed on a `VALIDATE_HASH` event
    /// Code: 35
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_validate_hash(&self, state: ApplicationState<T>);

    /// Executed on a `VALIDATED_HASH` event
    /// Code: 36
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_validated_hash(&self, state: ApplicationState<T>);

    /// Executed on a `FOUND_BLOCK` event
    /// Code: 37
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_found_block(&self, state: ApplicationState<T>);

    /// Executed on a `EXPLORE_NETWORK` event
    /// Code: 240
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_explore_network(&self, state: ApplicationState<T>);
}