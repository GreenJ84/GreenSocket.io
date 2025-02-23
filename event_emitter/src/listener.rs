use std::sync::Arc;

use crate::{Callback, EventPayload};

/// A Struct that is used to enact upon an emit Event
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
