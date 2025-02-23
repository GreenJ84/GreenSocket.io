use std::sync::Arc;

mod event_handler;
mod event_manager;
mod listener;

pub use listener::Listener;
pub use event_handler::EventHandler;
pub use event_manager::EventManager;

/// Type alias for anything that is cross-thread safe and send-able
pub type EventPayload<T: Send + Sync> = Arc<T>;
/// Type alias for any function that is cross-thread safe, send-able and takes a [`EventPayload`](EventPayload) with no return
pub type Callback<T: Send + Sync> = Arc<dyn Fn(EventPayload<T>) + Send + Sync>;

#[derive(Debug, Clone)]
/// Event Error enum for all customer and unknown error possibilities
pub enum EventError {
    /// Trying to add more than `max_listeners`to an Event.
    OverloadedEvent,
    /// Trying to delete a `Listener` that cannot be found.
    ListenerNotFound,
    /// Trying to delete an `Event` that cannot be found.
    EventNotFound,
    /// Any other possible Errors during Event Handling
    Other(Box<dyn std::error::Error + Send + Sync>),
}