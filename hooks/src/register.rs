use enums::EventCodes;

/// TODO:
pub trait Hooks {
    /// TODO:
    fn on_ping(&self);

    /// TODO:
    fn on_pong(&self);
}

/// Registers all events
pub struct HookRegister {
    hooks: Vec<Box<Hooks>>
}

impl HookRegister {
    /// Creates new empty handlers
    pub fn new() -> Self {
        Self {
            hooks: Vec::new()
        }
    }

    /// TODO:
    pub fn add_hook<H: Hooks + 'static>(mut self, hook: H) -> Self {
        self.hooks.push(Box::new(hook));
        self
    }

    /// TODO:
    pub fn notify(self, event: EventCodes) {
        for hook in self.hooks {
            match event {
                EventCodes::Ping => hook.on_ping(),
                EventCodes::Pong => hook.on_pong(),
                EventCodes::Register => hook.on_pong(),
                EventCodes::AckRegister => hook.on_pong(),
                EventCodes::PeerRegistering => hook.on_pong(),
                EventCodes::NewBlock => hook.on_pong(),
                EventCodes::PossibleBlock => hook.on_pong(),
                EventCodes::FoundBlock => hook.on_pong(),
                EventCodes::NotAValidEvent => hook.on_pong(),
            };
        }
    }
}
