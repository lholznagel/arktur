use hooks::Hooks;
use notification::HookNotification;

use std::sync::{Arc, Mutex};

/// Registers all hooks
#[derive(Debug)]
pub struct HookRegister<T> where T: Send {
    hook: Hooks<T>,
    state: Arc<Mutex<T>>
}

impl<T: 'static> HookRegister<T> where T: Send {
    /// Creates new empty handlers
    //pub fn new(hook: Box<Hooks<T>>, state: Arc<Mutex<T>>, ping: fn(MessageState<T>)) -> Self {
    pub fn new(hook: Hooks<T>, state: Arc<Mutex<T>>) -> Self {
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
    pub fn set_hook(mut self, hook: Hooks<T>) -> Self {
        self.hook = hook;
        self
    }

    /// Gets a new instance of hook notifications
    pub fn get_notification(self) -> HookNotification<T> {
        HookNotification::new(self.hook, self.state)
    }
}
