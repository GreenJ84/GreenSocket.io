///! A Copy of `rocket_cors` crate implementations to keep a decoupling from any framework specific ties

use std::collections::HashSet;
use http::Method;

pub enum AllOrSome<T> {
    All,
    Some(T),
}
impl<T> AllOrSome<T> {
    pub fn is_all(&self) -> bool{
        return if let Some(_) = self { false } else { true };
    }
    pub fn is_some(&self) -> bool{
        return if let Some(_) = self { true } else { false };
    }
    pub fn unwrap(self) -> Result<T, ()> {
        if let Some(t) = self {
            return Ok(t)
        }
        Err(())
    }
}

pub struct Origins {
    pub allow_null: bool,
    pub exact: Option<HashSet<String>>,
    pub regex: Option<HashSet<String>>,
}
pub type AllowedOrigins = AllOrSome<Origins>;

pub type AllowedMethods = HashSet<Method>;

pub type AllowedHeaders = AllOrSome<HashSet<String>>;
impl AllowedHeaders {
    pub fn all() -> Self {
        AllOrSome::All
    }
    pub fn some(list: &[&str]) -> Self{
        AllOrSome::Some(HashSet::from(list))
    }
}

pub struct CorsOptions {
    pub allowed_origins: AllowedOrigins,
    pub allowed_methods: AllowedMethods,
    pub allowed_headers: AllowedHeaders,
    pub allow_credentials: bool,
    pub expose_headers: HashSet<String>,
    pub max_age: Option<usize>,
    pub send_wildcard: bool,
    pub fairing_route_base: String,
    pub fairing_route_rank: isize,
}