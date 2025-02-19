use std::collections::HashSet;
use http::Method;
use url::ParseError;
use regex::Error as RegexError;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Eq, PartialEq, Clone, Debug)]
pub enum AllOrSome<T> {
    #[default]
    All,
    Some(T),
}
impl<T> AllOrSome<T> {
    pub fn all() -> Self {
        Self::All
    }
    pub fn some(items: &[T]) -> Self{
        Self::Some(items)
    }
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

pub enum CorsError {
    MissingCors,
    MissingOrigin,
    BadOrigin(ParseError),
    OpaqueAllowedOrigin(Vec<String>),
    MissingRequestMethod,
    BadRequestMethod,
    MissingRequestHeaders,
    OriginNotAllowed(String),
    MethodNotAllowed(String),
    RegexError(RegexError),
    HeadersNotAllowed,
    CredentialsWithWildcardOrigin,
}

#[derive(Default, Deserialize, Serialize, Eq, PartialEq, Clone, Debug)]
pub struct Origins {
    pub allow_null: bool,
    pub exact: Option<HashSet<String>>,
    pub regex: Option<HashSet<String>>,
}

pub type AllowedOrigins = AllOrSome<Origins>;
pub type AllowedMethods = HashSet<Method>;
pub type AllowedHeaders = AllOrSome<HashSet<String>>;

#[derive(Default, Deserialize, Serialize, Eq, PartialEq, Clone, Debug)]
pub struct CorsOptions {
    pub allowed_origins: AllowedOrigins,
    pub allowed_methods: AllowedMethods,
    pub allowed_headers: AllowedHeaders,
    pub allow_credentials: bool,
    pub expose_headers: HashSet<String>,
    pub max_age: Option<usize>,
    pub send_wildcard: bool,
}
impl CorsOptions {
    pub fn allowed_origins(mut self, allowed: AllowedOrigins) -> Self {
        self.allowed_origins = allowed;
        self
    }
    pub fn allowed_methods(mut self, allowed: AllowedMethods) -> Self {
        self.allowed_methods = allowed;
        self
    }
    pub fn allowed_headers(mut self, allowed: AllowedHeaders) -> Self {
        self.allowed_headers = allowed;
        self
    }
    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    }
    pub fn expose_headers(mut self, headers: HashSet<String>) -> Self {
        self.expose_headers = headers;
        self
    }
    pub fn max_age(mut self, max_age: usize) -> Self {
        self.max_age = if max_age > 0 { Some(max_age) } else { None };
        self
    }
    pub fn send_wildcard(mut self, send: bool) -> Self {
        self.send_wildcard = send;
        self
    }
}