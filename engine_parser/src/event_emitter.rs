use std::sync::Arc;
use dashmap::DashMap;
use futures::future::BoxFuture;

pub type EventPayload<T: Send + Sync> = Arc<T>;
pub type Callback<T: Send + Sync> = Arc<dyn Fn(EventPayload<T>) + Send + Sync>;
#[derive(Debug, Clone)]
pub struct Listener<T> {
    callback: Callback<T>,
    lifetime: Option<u64>,
}
impl<T> Listener<T> {
    pub fn new<T>(callback: Arc<Callback<T>>, lifetime: Option<u64>) -> Self {
        Self { callback, lifetime }
    }
    pub fn call<T>(&self, payload: EventPayload<T>){
        self.callback(payload);
        if let Some(mut lifetime) = self.lifetime{
            *lifetime -= 1;
        }
    }
}

impl<T> PartialEq<Self> for Listener<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.callback, &other.callback)
    }
}
impl<T> Eq for Listener<T>{}

#[derive(Debug, Clone)]
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

#[derive(Default, Debug, Clone)]
pub struct EventEmitter<T> {
    max_listeners: usize,
    listeners: Arc<DashMap<String, Vec<Listener<T>>>>,
}
impl<T> EventEmitter<T>{
    pub fn new() -> Self {
        Self {
            max_listeners: 10usize,
            listeners: Arc::new(DashMap::new()),
        }
    }
}

impl<T> EventHandler<T> for EventEmitter<T> {
    fn event_names(&self) -> Vec<String> {
        self.listeners.iter().map(|entry| entry.key().clone()).collect()
    }

    fn add_listener<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError> {
        let mut entry = self.listeners.entry(event_name.to_string()).or_default();
        if entry.len() < self.max_listeners {
            entry.push(callback);
            Ok(())
        } else {
            Err(EventError::OverloadedEvent)
        }
    }

    fn on<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError> {
        self.add_listener(event_name, callback)
    }

    fn add_limited_listener<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError> {
        todo!()
    }

    fn on_limited<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError> {
        todo!()
    }

    fn once<T: Send + Sync>(&mut self, event_name: &str, callback: Listener<T>) -> Result<(), EventError> {
        let mut entry = self.listeners.entry(event_name.to_string()).or_default();
        if entry.len() < self.max_listeners {
            entry.push(callback);
            Ok(())
        } else {
            Err(EventError::OverloadedEvent)
        }
    }

    fn remove_listener<T: Send + Sync>(&mut self, event_name: &str, callback: &Listener<T>) -> Result<(), EventError> {
        if let Some(mut entry) = self.listeners.get_mut(event_name) {
            let original_len = entry.len();
            entry.retain(|listener| !Arc::ptr_eq(listener, callback));

            return if entry.len() < original_len {
                Ok(())
            } else {
                Err(EventError::ListenerNotFound)
            };
        }
        Err(EventError::EventNotFound)
    }

    fn off<T: Send + Sync>(&mut self, event_name: &str, callback: &Listener<T>) -> Result<(), EventError> {
        self.remove_listener(event_name, callback)
    }

    fn remove_all_listeners(&mut self, event_name: &str) -> Result<(), EventError> {
        if self.listeners.remove(event_name).is_some() {
            Ok(())
        } else {
            Err(EventError::EventNotFound)
        }
    }

    fn emit<T: Send + Sync>(&self, event_name: &str, payload: EventPayload<T>) {
        if let Some(entry) = self.listeners.get(event_name) {
            for listener in entry.iter() {
                listener.callback(payload.clone());
            }
        }
    }

    async fn emit_async<T: Send + Sync>(&self, event_name: &str, payload: EventPayload<T>) {
        let clone = self.clone();
        tokio::spawn(async move {
            clone.emit(event_name, payload);
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