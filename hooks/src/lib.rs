mod empty;
mod event_codes;
mod hooks;
mod notification;
mod register;

pub use event_codes::{as_enum, as_number, EventCodes};
pub use hooks::Hooks;
pub use notification::HookNotification;
pub use register::HookRegister;