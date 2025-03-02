use std::future::Future;
use std::sync::Arc;
use dashmap::DashMap;
use crate::event_handler::EventHandler;
use crate::{Callback, EventError, EventPayload};
use crate::listener::Listener;

pub type EventManager<T> = Arc<EventEmitter<T>>;

/// A struct intended to handle the implementations of reacting to Events
#[derive(Clone)]
pub struct EventEmitter<T> where T: Send + Sync + 'static  {
    max_listeners: usize,
    listeners: Arc<DashMap<String, Vec<Listener<T>>>>,
}
impl<T: Send + Sync+ 'static> EventEmitter<T>{
    pub fn new(max_listeners: usize) -> Self {
        Self {
            max_listeners,
            listeners: Arc::new(DashMap::new()),
        }
    }
}
impl<T: Send + Sync> Default for EventEmitter<T>{
    fn default() -> Self {
        Self {
            max_listeners: 10,
            listeners: Arc::new(DashMap::new()),
        }
    }
}
impl<T: Send + Sync> EventEmitter<T> {
    fn listeners_mut(&mut self) -> &Arc<DashMap<String, Vec<Listener<T>>>> {
        &self.listeners
    }
}
impl<T: Send+ Sync> EventHandler<T> for EventEmitter<T> {
    fn event_names(&self) -> Vec<String> {
        self.listeners.iter().map(|entry| entry.key().clone()).collect()
    }
    fn set_max_listeners(&mut self, max: usize) { self.max_listeners = max; }
    fn max_listeners(&self) -> usize { self.max_listeners }


    fn add_listener(&mut self, event_name: &str, callback: Callback<T>) -> Result<Listener<T>, EventError> {
        self.add_limited_listener(event_name, callback, 0)
    }

    fn add_limited_listener(&mut self, event_name: &str, callback: Callback<T>, limit: u64) -> Result<Listener<T>, EventError> {
        let mut entry = self.listeners.entry(event_name.to_string()).or_default();
        if entry.len() < self.max_listeners {
            let listener = Listener::new(
                callback,
                if limit > 0 { Some(limit) } else { None }
            );
            entry.push(listener.clone());
            return Ok(listener);
        }
        Err(EventError::OverloadedEvent)
    }

    fn add_once(&mut self, event_name: &str, callback: Callback<T>) -> Result<Listener<T>, EventError> {
        self.add_limited_listener(event_name, callback, 1)
    }


    fn listener_count(&self, event_name: &str) -> usize {
        self.listeners
            .get(event_name)
            .map(|entry| entry.len())
            .unwrap_or(0)
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

    fn remove_all_listeners(&mut self, event_name: &str) -> Result<(), EventError> {
        if self.listeners.remove(event_name).is_some() {
            Ok(())
        } else {
            Err(EventError::EventNotFound)
        }
    }


    fn emit(&mut self, event_name: &str, payload: EventPayload<T>) -> Result<(), EventError> {
        if let Some(mut entry) = self.listeners_mut().get_mut(event_name) {
            for idx in (0..entry.len()).rev() {
                let listener = entry.get(idx).unwrap();
                listener.call(&payload);
                if listener.at_limit(){
                    entry.remove(idx);
                }
            }
            return Ok(());
        }
        Err(EventError::EventNotFound)
    }

    fn emit_final(&mut self, event_name: &str, payload: EventPayload<T>) -> Result<(), EventError> {
        if self.listeners_mut().contains_key(event_name){
            for listener in self.listeners.get(event_name).unwrap().iter() {
                listener.call(&payload);
            }
            self.listeners_mut().remove(event_name);
            return Ok(())
        }
        Err(EventError::EventNotFound)
    }

    fn emit_async<'a>(&'a mut self, event_name: &'a str, payload: EventPayload<T>) -> Box<dyn Future<Output=()> + Send + 'a> {
        Box::new(async move {
            if let Some(entry) = self.listeners.get(event_name) {
                for listener in entry.iter() {
                    let mut _self = self.clone();
                    let listener = listener.clone();
                    let payload = Arc::clone(&payload);
                    let event_name = event_name.to_string();
                    tokio::spawn(async move {
                        listener.call(&payload);
                        if listener.at_limit() {
                            if let Err(err) =  _self.remove_listener(&event_name, &listener){
                                eprintln!("Error removing listener: {:?}", err);
                            }
                        }
                    });
                }
            }
        })
    }

    fn emit_final_async<'a>(&'a mut self, event_name: &'a str, payload: EventPayload<T>) -> Box<dyn Future<Output=()> + Send + 'a> {
        Box::new(async move {
            if let Some(entry) = self.listeners.get(event_name) {
                for listener in entry.iter() {
                    let listener = listener.clone();
                    let payload = Arc::clone(&payload);
                    tokio::spawn(async move {
                        listener.call(&payload);
                    });
                }
                self.listeners.remove(event_name);
            }
        })
    }
}