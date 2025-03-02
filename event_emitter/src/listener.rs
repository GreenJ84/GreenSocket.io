use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

use crate::{Callback, EventPayload};

/// A Struct that is used to enact upon an emit Event
pub struct Listener<T> {
    callback: Callback<T>,
    lifetime: Option<Arc<Mutex<u64>>>,
}
impl<T: Send + Sync + 'static> Listener<T> {
    pub fn new(callback: Callback<T>, lifetime: Option<u64>) -> Self {
        if let Some(limit) = lifetime {
            return Self { callback, lifetime: Some(Arc::new(Mutex::new(limit))) };
        }
        Self { callback, lifetime: None }
    }

    /// Best for Synchronous order dependant tasks
    pub fn call(&mut self, payload: &EventPayload<T>) {
        if let Some(ref lifetime) = self.lifetime {
            let mut count = lifetime.lock().unwrap();
            *count -= 1;
        }
        (self.callback)(payload);
    }

    /// Best for Background Processing and I/O heavy tasks
    pub fn background_call(&mut self, payload: &EventPayload<T>) {
        if let Some(ref lifetime) = self.lifetime {
            let mut count = lifetime.lock().unwrap();
            *count -= 1;
        }

        tokio::spawn({
            let callback = Arc::clone(&self.callback);
            let payload = Arc::clone(&payload);
            async move {
                callback(&payload);
            }
        });
    }

    /// Best for CPU heavy Operations
    pub fn blocking_call(&mut self, payload: &EventPayload<T>) {
        if let Some(ref lifetime) = self.lifetime {
            let mut count = lifetime.lock().unwrap();
            *count -= 1;
        }

        tokio::task::spawn_blocking({
            let callback = Arc::clone(&self.callback);
            let payload = Arc::clone(&payload);
            move || { callback(&payload) }
        });
    }

    pub fn at_limit(&self) -> bool {
        if let Some(ref lifetime) = self.lifetime {
            let count = lifetime.lock().unwrap();
            return *count == 0;
        }
        false
    }
    pub fn eq_callback(&self, callback: Callback<T>) -> bool{
        Arc::ptr_eq(&self.callback, &callback)
    }
}
impl<T> Clone for Listener<T> {
    fn clone(&self) -> Self {
        Self {
            callback: Arc::clone(&self.callback),
            lifetime: if let Some(limit) = &self.lifetime {
                Some(Arc::clone(limit))
            } else { None },
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.callback = Arc::clone(&source.callback);
        self.lifetime = if let Some(limit) = &source.lifetime {
            Some(Arc::clone(limit))
        } else { None };
    }
}
impl<T> Debug for Listener<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Listener (limit: {:?})", self.lifetime)
    }
}
impl<T> PartialEq<Self> for Listener<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.callback, &other.callback)
    }
}
impl<T> Eq for Listener<T>{}
