use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

use crate::{Callback, EventPayload};

/// A Struct that is used to enact upon an emit Event
pub struct Listener<T> {
    callback: Callback<T>,
    lifetime: Option<Arc<Mutex<u64>>>,
}
impl<T> Listener<T> {
    pub fn new(callback: Callback<T>, lifetime: Option<u64>) -> Self {
        Self { callback, lifetime }
    }
    pub fn call(&self, payload: &EventPayload<T>) {
       (self.callback)(payload);
        if let Some(mut _lifetime) = self.lifetime{
            _lifetime -= 1;
        }
    }
    pub fn at_limit(&self) -> bool {
        self.lifetime == Some(0)
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
