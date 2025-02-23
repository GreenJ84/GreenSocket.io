use crate::{EventError, EventPayload};
use crate::listener::Listener;
use futures::future::BoxFuture;

/// A generic trait with functionality required for any struct that intends to react to events in accordance to this package
pub trait EventHandler<T> {
    fn event_names(&self) -> Vec<String>;

    /// Infinite Listener
    fn add_listener<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError>;
    fn on<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError>;
    /// Finite Listener
    fn add_limited_listener<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError>;
    fn on_limited<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError>;
    /// Single Instance Listener
    fn once<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError>;
    /// Remove an active listener for an Event
    fn remove_listener<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError>;
    fn off<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError>;
    /// Remove all Listeners for an Event
    fn remove_all_listeners(&mut self, event_name: &str) -> Result<(), EventError>;

    fn emit<T: Send + Sync>(&self, event_name: &str, payload: EventPayload<T>);
    async fn emit_async<T: Send + Sync>(&self, event_name: &str, payload: EventPayload<T>);

    fn emit_final<T: Send + Sync>(&mut self, event_name: &str, payload: EventPayload<T>) -> Result<(), EventError>{
        self.emit(event_name, payload);
        self.remove_all_listeners(event_name)
    }
    fn emit_final_async<T: Send + Sync>(&mut self, event_name: &str, payload: EventPayload<T>) -> BoxFuture<'_, Result<(), EventError>> {
        Box::pin(async move {
            self.emit_async(event_name, payload).await;
            self.remove_all_listeners(event_name)
        })
    }

    fn set_max_listeners(&mut self, max: usize);
    fn max_listeners(&self) -> usize;
    fn listener_count(&self, event_name: &str) -> Result<usize, EventError>;
}