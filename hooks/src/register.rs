use hooks::Hooks;
use notification::HookNotification;

use std::sync::{Arc, Mutex};

/// Registers all events
pub struct HookRegister<T> {
    hook: Box<Hooks<T>>,
    state: Arc<Mutex<T>>
}

impl<T> HookRegister<T> {
    /// Creates new empty handlers
    pub fn new(hook: Box<Hooks<T>>, state: Arc<Mutex<T>>) -> Self {
        Self {
            hook,
            state
        }
    }

    /// Sets the hook
    ///
    /// # Parameters
    ///
    /// - `hook` - Struct that implements the `Hooks` trait
    pub fn set_hook<H: Hooks<T> + 'static>(mut self, hook: H) -> Self {
        self.hook = Box::new(hook);
        self
    }

    pub fn get_notification(self) -> HookNotification<T> {
        HookNotification::new(self.hook, self.state)
    }
}
