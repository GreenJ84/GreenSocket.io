use std::sync::Arc;

mod event_handler;
mod event_emitter;
mod listener;

#[cfg(test)]
mod tests;

pub use listener::Listener;
pub use event_handler::EventHandler;
pub use event_emitter::{
    EventManager,
    EventEmitter
};

/// Type alias for anything that is cross-thread safe and send-able
pub type EventPayload<T> = Arc<T>;
/// Type alias for any function that is cross-thread safe, send-able and takes a [`EventPayload`](EventPayload) with no return
pub type Callback<T> = Arc<dyn Fn(&EventPayload<T>) + Send + Sync>;

#[derive(Debug)]
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
impl PartialEq for EventError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EventError::ListenerNotFound, EventError::ListenerNotFound) |
            (EventError::EventNotFound, EventError::EventNotFound) |
            (EventError::OverloadedEvent, EventError::OverloadedEvent) => {
                true
            },
            (EventError::Other(a), EventError::Other(b)) => a.to_string() == b.to_string(),
            _=> false
        }
    }
}
impl Eq for EventError {}