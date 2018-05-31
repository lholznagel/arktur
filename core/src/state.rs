use config::Config;

#[derive(Clone, Debug)]
pub struct State {
    pub config: Config
}

impl State {

    pub fn new(config: Config) -> Self {
        Self {
            config
        }
    }
}

#[derive(Debug)]
/// Builder for constructing the application state
pub struct StateBuilder {
    config: Config,
}

impl StateBuilder {

    /// Creates a default builder
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// Sets the configuration
    pub fn set_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    /// Creates a new state
    pub fn build(self) -> State {
        State::new(self.config)
    }
}
