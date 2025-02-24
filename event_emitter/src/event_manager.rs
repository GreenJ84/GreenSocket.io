use std::sync::Arc;
use dashmap::DashMap;
use crate::event_handler::EventHandler;
use crate::{EventError, EventPayload};
use crate::listener::Listener;

pub type EventManager<T> = Arc<EventEmitter<T>>;

/// A struct intended to handle the implementations of reacting to Events
#[derive(Default, Clone)]
struct EventEmitter<T> where T: Send + Sync + Clone  {
    max_listeners: usize,
    listeners: Arc<DashMap<String, Vec<Listener<T>>>>,
}
impl<T: Send + Sync + Clone> EventEmitter<T>{
    pub fn new() -> Self {
        Self {
            max_listeners: 10usize,
            listeners: Arc::new(DashMap::new()),
        }
    }
}
impl<T: Send+ Sync + Clone + 'static> EventHandler<T> for EventEmitter<T> {
    fn event_names(&self) -> Vec<String> {
        self.listeners.iter().map(|entry| entry.key().clone()).collect()
    }

    fn add_listener(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError> {
        let mut entry = self.listeners.entry(event_name.to_string()).or_default();
        if entry.len() < self.max_listeners {
            entry.push(callback);
            Ok(())
        } else {
            Err(EventError::OverloadedEvent)
        }
    }

    fn on(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError> {
        self.add_listener(event_name, callback)
    }

    fn add_limited_listener(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError> {
        todo!()
    }

    fn on_limited(&mut self, event_name: &str, callback: Listener<T>, limit: u64) -> Result<(), EventError> {
        todo!()
    }

    fn once(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError> {
        let mut entry = self.listeners.entry(event_name.to_string()).or_default();
        if entry.len() < self.max_listeners {
            entry.push(callback);
            Ok(())
        } else {
            Err(EventError::OverloadedEvent)
        }
    }

    fn remove_listener(&mut self, event_name: &str, callback: &Listener<T>) -> Result<(), EventError> {
        if let Some(mut entry) = self.listeners.get_mut(event_name) {
            let original_len = entry.len();
            entry.retain(|listener| !listener.eq(callback));

            return if entry.len() < original_len {
                Ok(())
            } else {
                Err(EventError::ListenerNotFound)
            };
        }
        Err(EventError::EventNotFound)
    }

    fn off(&mut self, event_name: &str, callback: &Listener<T>) -> Result<(), EventError> {
        self.remove_listener(event_name, callback)
    }

    fn remove_all_listeners(&mut self, event_name: &str) -> Result<(), EventError> {
        if self.listeners.remove(event_name).is_some() {
            Ok(())
        } else {
            Err(EventError::EventNotFound)
        }
    }

    fn emit(&mut self, event_name: &str, payload: EventPayload<T>) {
        if let Some(entry) = self.listeners.get(event_name) {
            for listener in entry.iter() {
                let mut _self = self.clone();
                let payload = payload.clone();
                let listener = listener.clone();
                let event_name = event_name.to_string();
                tokio::spawn(async move {
                    listener.call(payload);
                    if listener.at_limit() {
                        _self.remove_listener(&event_name, &listener).map_err(|_|{
                            eprintln!("Failed to drop Listener at call limit");
                        }).ok().unwrap();
                    }
                });
            }
        }
    }

    async fn emit_async(&mut self, event_name: &str, payload: EventPayload<T>) {
        let _event_name = event_name.to_string();
        let mut self_clone = self.clone();

        tokio::spawn(async move {
            self_clone.emit(&_event_name, payload);
        });
    }

    fn set_max_listeners(&mut self, max: usize) {
        self.max_listeners = max;
    }

    fn max_listeners(&self) -> usize {
        self.max_listeners
    }

    fn listener_count(&self, event_name: &str) -> Result<usize, EventError> {
        self.listeners
            .get(event_name)
            .map(|entry| entry.len())
            .ok_or(EventError::EventNotFound)
    }
}