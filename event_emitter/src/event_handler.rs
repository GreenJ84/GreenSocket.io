use std::pin::Pin;
use crate::{EventError, EventPayload};
use crate::listener::Listener;

/// A generic trait with functionality required for any struct that intends to react to events in accordance to this package
pub trait EventHandler<T: Send + Sync + Clone>: Send + Sync + Clone {
    fn event_names(&self) -> Vec<String>;

    /// Infinite Listener
    fn add_listener(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError>;
    fn on(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError>;
    /// Finite Listener
    fn add_limited_listener(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError>;
    fn on_limited(&mut self, event_name: &str, callback: Listener<T>, limit: u64) -> Result<(), EventError>;
    /// Single Instance Listener
    fn once(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError>;
    /// Remove an active listener for an Event
    fn remove_listener(&mut self, event_name: &str, callback: &Listener<T>) -> Result<(), EventError>;
    fn off(&mut self, event_name: &str, callback: &Listener<T>) -> Result<(), EventError>;
    /// Remove all Listeners for an Event
    fn remove_all_listeners(&mut self, event_name: &str) -> Result<(), EventError>;

    fn emit(&mut self, event_name: &str, payload: EventPayload<T>);
    async fn emit_async(&mut self, event_name: &str, payload: EventPayload<T>);

    fn emit_final(&mut self, event_name: &str, payload: EventPayload<T>) -> Result<(), EventError>{
        self.emit(event_name, payload);
        self.remove_all_listeners(event_name)
    }
    fn emit_final_async(&mut self, event_name: &str, payload: EventPayload<T>) -> Pin<Box<dyn futures::Future<Output = Result<(), EventError>>>> {
        Box::pin(async move {
            self.emit_async(event_name, payload).await;
            self.remove_all_listeners(event_name)
        })
    }

    fn set_max_listeners(&mut self, max: usize);
    fn max_listeners(&self) -> usize;
    fn listener_count(&self, event_name: &str) -> Result<usize, EventError>;
}