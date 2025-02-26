use std::future::Future;
use crate::{Callback, EventError, EventPayload};
use crate::listener::Listener;

/// A generic trait with functionality required for any struct that intends to react to events in accordance to this package
pub trait EventHandler<T: Send + Sync>: Send + Sync {
    fn event_names(&self) -> Vec<String>;
    fn set_max_listeners(&mut self, max: usize);
    fn max_listeners(&self) -> usize;

    /// Add an infinite Listener
    fn add_listener(&mut self, event_name: &str, callback: Callback<T>) -> Result<(), EventError>;
    /// Add a finite Listener
    fn add_limited_listener(&mut self, event_name: &str, callback: Callback<T>, limit: u64) -> Result<(), EventError>;
    /// Add a single instance Listener
    fn add_once(&mut self, event_name: &str, callback: Callback<T>) -> Result<(), EventError>;

    /// Check the number of listeners that are registered to a certain event<br/>
    /// Throws an error for event names that have no active listeners (are not currently registered)
    fn listener_count(&self, event_name: &str) ->  usize;
    /// Check if an event has any registered listeners
    fn has_listener(&self, event_name: &str) ->  bool {
        self.listener_count(event_name) > 0
    }

    /// Remove an active listener for an Event
    fn remove_listener(&mut self, event_name: &str, callback: &Listener<T>) -> Result<(), EventError>;
    /// Remove all Listeners for an Event
    fn remove_all_listeners(&mut self, event_name: &str) -> Result<(), EventError>;

    /// Ensure finality/success of all event callbacks
    fn emit(&mut self, event_name: &str, payload: EventPayload<T>) -> Result<(), EventError>;
    /// Ensure finality/success of all event callbacks before deleting the event
    fn emit_final(&mut self, event_name: &str, payload: EventPayload<T>) -> Result<(), EventError>;
    /// Fire and forget all event callbacks
    fn emit_async<'a>(&'a mut self, event_name: &'a str, payload: EventPayload<T>) -> Box<dyn Future<Output=()> + Send + 'a>;
    /// Fire and forget all event callbacks before deleting the event
    fn emit_final_async<'a>(&'a mut self, event_name: &'a str, payload: EventPayload<T>) -> Box<dyn Future<Output=()> + Send + 'a>;
}