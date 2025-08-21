use std::collections::HashSet;
use http::{Method, StatusCode};
use hyper::service::Service;
use url::ParseError;
use regex::Error as RegexError;
use serde::{Deserialize, Serialize};
use engine_parser::RawData;

use crate::transports::{EngineRequest, EngineResponse};

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
pub enum AllOrSome<T> {
    #[default]
    All,
    Some(T),
}
impl<T> AllOrSome<T> {
    pub fn all() -> Self {
        Self::All
    }
    pub fn some(items: T) -> Self {
        Self::Some(items)
    }
    pub fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }
    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }
    pub fn unwrap(self) -> Result<T, ()> {
        match self {
            Self::Some(t) => Ok(t),
            Self::All => Err(())
        }
    }
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

#[derive(Clone)]
pub struct CorsMiddleware {
    options: CorsOptions,
}

impl Service<EngineRequest> for CorsMiddleware {
    type Response = EngineResponse;
    type Error = hyper::Error;
    type Future = Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>> + Send>;

    fn call(&mut self, req: EngineRequest) -> Self::Future {
        if self.is_preflight_request(&req) {
            let mut res = EngineResponse::new(RawData::Text("Ok".to_string()));
            let cors_headers = self.generate_cors_headers(&req);
            for (name, value) in cors_headers {
                if let (Ok(header_name), Ok(header_value)) = (name.parse(), value.parse()) {
                    res.headers_mut().insert(header_name, header_value);
                }
            }
            *res.status_mut() = StatusCode::OK;
            return Box::new(futures::future::ok(res));
        }

        // TODO: This needs to be implemented to call the next middleware/handler
        // For now, return a basic response to prevent infinite recursion
        let mut res = EngineResponse::new(RawData::Text("Not Implemented".to_string()));
        let cors_headers = self.generate_cors_headers(&req);
        for (name, value) in cors_headers {
            if let (Ok(header_name), Ok(header_value)) = (name.parse(), value.parse()) {
                res.headers_mut().insert(header_name, header_value);
            }
        }
        *res.status_mut() = StatusCode::NOT_IMPLEMENTED;
        Box::new(futures::future::ok(res))
    }
}

impl CorsMiddleware{
    pub fn new(options: CorsOptions) -> Self {
        Self { options }
    }

    fn generate_cors_headers(&self, req: &EngineRequest) -> Vec<(String, String)> {
        let mut headers = Vec::new();

        // Access-Control-Allow-Origin - VALIDATE BEFORE SETTING
        if let Some(origin_header) = req.headers().get("Origin") {
            let origin_str = origin_header.to_str().unwrap_or_default();
            
            // Validate the origin before setting the header
            match self.validate_origin(origin_str) {
                Ok(validated_origin) => {
                    if self.options.send_wildcard && matches!(self.options.allowed_origins, AllOrSome::All) {
                        headers.push(("Access-Control-Allow-Origin".to_string(), "*".to_string()));
                    } else {
                        headers.push(("Access-Control-Allow-Origin".to_string(), validated_origin.to_string()));
                    }
                }
                Err(_) => {
                    // Don't set the header if origin is not allowed
                    // This prevents CORS attacks
                }
            }
        }

        // Access-Control-Allow-Methods
        headers.push((
            "Access-Control-Allow-Methods".to_string(),
            self.options.allowed_methods.iter().map(|m| m.as_str()).collect::<Vec<_>>().join(", ")
        ));

        // Access-Control-Allow-Headers
        let allowed_headers = match &self.options.allowed_headers {
            AllOrSome::All => "*".to_string(),
            AllOrSome::Some(headers) => headers.iter().cloned().collect::<Vec<_>>().join(", "),
        };
        headers.push(("Access-Control-Allow-Headers".to_string(), allowed_headers));

        // Access-Control-Allow-Credentials - SECURITY: Never allow wildcard origin with credentials
        if self.options.allow_credentials {
            if !self.options.send_wildcard {
                headers.push(("Access-Control-Allow-Credentials".to_string(), "true".to_string()));
            }
            // Silently ignore credentials=true when wildcard is used (security best practice)
        }

        // Access-Control-Expose-Headers
        if !self.options.expose_headers.is_empty() {
            headers.push((
                "Access-Control-Expose-Headers".to_string(),
                self.options.expose_headers.iter().cloned().collect::<Vec<_>>().join(", "),
            ));
        }

        // Access-Control-Max-Age
        if let Some(max_age) = self.options.max_age {
            headers.push(("Access-Control-Max-Age".to_string(), max_age.to_string()));
        }

        headers
    }

    fn is_preflight_request(&self, req: &EngineRequest) -> bool {
        req.method() == Method::OPTIONS &&
        req.headers().contains_key("Origin") &&
        req.headers().contains_key("Access-Control-Request-Method")
    }

    fn validate_origin(&self, origin: &str) -> Result<&str, CorsError> {
        match &self.options.allowed_origins {
            AllOrSome::All => Ok(origin),
            AllOrSome::Some(origins) => {
                if origin.is_empty() && origins.allow_null { return Ok(origin); }
                if let Some(exact) = &origins.exact {
                    if exact.contains(origin) { return Ok(origin); }
                }
                if let Some(regex) = &origins.regex {
                    // Check against regex - handle compilation errors safely
                    let matches = regex.iter().any(|r| {
                        match regex::Regex::new(r) {
                            Ok(compiled_regex) => compiled_regex.is_match(origin),
                            Err(_) => {
                                // Log the error but don't crash - treat as no match
                                eprintln!("Warning: Invalid regex pattern: {}", r);
                                false
                            }
                        }
                    });
                    
                    if matches {
                        Ok(origin)
                    } else {
                        Err(CorsError::OriginNotAllowed(origin.to_string()))
                    }
                } else {
                    Err(CorsError::OriginNotAllowed(origin.to_string()))
                }
            }
        }
    }


}
