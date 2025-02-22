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

pub struct EventEmitter {
    max_listeners: usize,
    listeners: Arc<Mutex<HashMap<String, Vec<Listener>>>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            max_listeners: 10usize,
            listeners: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl EventHandler for EventEmitter {
    fn event_names(&self) -> Vec<String> {
        self.listeners.lock().unwrap().keys().map(|k| k.to_owned()).collect::<Vec<String>>()
    }

    fn add_listener(&mut self, event_name: &str, callback: Listener) -> Result<(), EventError>{
        let mut listeners = self.listeners.lock().unwrap();
        let event_listeners = listeners
            .entry(event_name.to_string())
            .or_insert(Vec::<Listener>::with_capacity(self.max_listeners));
        if event_listeners.len() < self.max_listeners {
            event_listeners.push(callback);
            return Ok(())
        }
        Err(EventError::MaxedEventListeners)
    }
    fn remove_listener(&mut self, event_name: &str, callback: Listener) -> Result<(), EventError>{
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(event_listeners) = listeners.get_mut(event_name){
            let original_len = event_listeners.len();
            event_listeners.retain(|listener| !Arc::ptr_eq(listener, &callback));

            return if event_listeners.len() < original_len {
                Ok(())
            } else {
                Err(EventError::ListenerNotFound)
            }
        }
        Err(EventError::EventNotFound)
    }
    fn remove_all_listeners(&mut self, event_name: &str) -> Result<(), EventError>{
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(event_listeners) = listeners.get_mut(event_name){
            event_listeners.clear();
            return Ok(())
        }
        Err(EventError::EventNotFound)
    }
    fn on(&mut self, event_name: &str, callback: Listener) -> Result<(), EventError> {
        self.add_listener(event_name, callback)
    }
    fn off(&mut self, event_name: &str, callback: Listener) -> Result<(), EventError> {
        self.remove_listener(event_name, callback)
    }

    fn emit(&self, event: &str, payload: EventPayload) {
        let listeners = self.listeners.lock().unwrap();
        if let Some(callbacks) = listeners.get(event) {
            for callback in callbacks {
                callback(payload.clone());
            }
        }
    }

    fn set_max_listeners(&mut self, max: usize) {
        self.max_listeners = max;
    }
    fn max_listeners(&self) -> usize {
        self.max_listeners
    }
    fn listener_count(&self, event_name: &str) -> Result<usize, EventError> {
        let listeners = self.listeners.lock().unwrap();
        if let Some(event_listeners) = listeners.get(event_name) {
            return Ok(event_listeners.len());
        }
        Err(EventError::EventNotFound)
    }

}
