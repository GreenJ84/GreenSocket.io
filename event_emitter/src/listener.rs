use std::sync::Arc;

use crate::{Callback, EventPayload};

/// A Struct that is used to enact upon an emit Event
#[derive(Clone)]
pub struct Listener<T> {
    callback: Callback<T>,
    lifetime: Option<u64>,
}
impl<T> Listener<T> {
    pub fn new(callback: Callback<T>, lifetime: Option<u64>) -> Self {
        Self { callback, lifetime }
    }
    pub fn call(&self, payload: &EventPayload<T>) {
       (self.callback)(payload);
        if let Some(mut lifetime) = self.lifetime{
            lifetime -= 1;
        }
    }
    pub fn at_limit(&self) -> bool {
        self.lifetime == Some(0)
    }
    pub fn eq_callback(&self, callback: Callback<T>) -> bool{
        Arc::ptr_eq(&self.callback, &callback)
    }
}

impl<T> PartialEq<Self> for Listener<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.callback, &other.callback)
    }
}
impl<T> Eq for Listener<T>{}
