use empty::Empty;
use hooks::Hooks;
use notification::HookNotification;

/// Registers all events
pub struct HookRegister {
    hook: Box<Hooks>
}

impl HookRegister {
    /// Creates new empty handlers
    pub fn new() -> Self {
        Self {
            hook: Box::new(Empty)
        }
    }

    /// Sets the hook
    ///
    /// # Parameters
    ///
    /// - `hook` - Struct that implements the `Hooks` trait
    pub fn set_hook<H: Hooks + 'static>(mut self, hook: H) -> Self {
        self.hook = Box::new(hook);
        self
    }

    pub fn get_notification(self) -> HookNotification {
        HookNotification::new(self.hook)
    }
}
