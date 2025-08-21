pub mod u_server;
pub mod server;

use http::{Error};
use engine_parser::{EventEmitter, EventError, EventHandler, EventPayload, Listener, Packet};
use std::collections::HashMap;

use crate::cookie::{CookieSerializeOptions, SameSiteSetting};
use crate::cors::{CorsMiddleware, CorsOptions};
use crate::socket::Socket;
use crate::transports::{EngineRequest, EngineResult, Transport, TransportType};

#[derive(Clone, Debug)]
pub struct AttachOptions {
    /// Name of the path to capture
    ///
    /// Default: "/engine.io"
    path: String,

    /// Destroy unhandled upgrade requests
    ///
    /// Default: true
    destroy_upgrade: bool,

    /// Milliseconds after which unhandled requests are ended
    ///
    /// Default: 1000
    destroy_upgrade_timeout: u32,

    /// Whether we should add a trailing slash to the request path.
    ///
    /// Default: true
    add_trailing_slash: bool
}
impl Default for AttachOptions {
    fn default() -> Self {
        Self {
            path: "/engine.greeno".to_string(),
            destroy_upgrade: true,
            destroy_upgrade_timeout: 1_000,
            add_trailing_slash: true,
        }
    }
}
impl AttachOptions {
    fn new(path: Option<String>, destroy_upgrade: Option<bool>, destroy_upgrade_timeout: Option<u32>, add_trailing_slash: Option<bool>) -> Self{
        Self {
            path: path.unwrap_or("/engine.greeno".to_string()),
            destroy_upgrade: destroy_upgrade.unwrap_or(true),
            destroy_upgrade_timeout: destroy_upgrade_timeout.unwrap_or(1_000),
            add_trailing_slash: add_trailing_slash.unwrap_or(true),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServerOptions {
    /// How many ms without a pong packet to consider the connection closed
    ///
    /// Default: 20_000
    pub ping_timeout: u32,

    ///how many ms before sending a new ping packet
    ///
    ///Default: 25_000
    pub ping_interval: u32,

    /// How many ms before an uncompleted transport upgrade is cancelled
    ///
    /// Default: 10_000
    pub upgrade_timeout: u32,

    /// How many bytes or characters a message can be, before closing the session (to avoid DoS).
    ///
    /// Default: 1e5 (100 KB)
    pub max_buffer_size: u32,

    /// A function that receives a given handshake or upgrade request as its first parameter,
    /// and can decide whether to continue or not. The second argument is a function that needs
    /// to be called with the decided information: fn(err, success), where success is a boolean
    /// value where false means that the request is rejected, and err is an error code.
    pub allow_request: Option<dyn Fn(
        EngineRequest,
        dyn Fn(Result<(), String>),
    )>,

    ///The low-level transports that are enabled. WebTransport is disabled by default and must be manually enabled:
    ///
    /// Default: `vec![TransportType::Polling, TransportType::WebSocket]`
    pub transports: Vec<TransportType>,

    /// Whether to allow transport upgrades
    ///
    /// Default: true
    pub allow_upgrades: bool,

    /// Enable WebSocket per-message-deflate extension
    ///
    /// Default: false
    pub per_message_deflate: Option<u32>,

    /// Enable http compression for the polling transports
    ///
    /// Default: true
    pub http_compression: bool,

    /// An optional packet which will be concatenated to the handshake packet emitted by Engine.IO.
    ///
    /// Default: None
    pub initial_packet: Option<Packet>,

    /// Configuration of the cookie that contains the client sid to send as part of handshake response headers. This cookie
    /// might be used for sticky-session. Defaults to not sending any cookie.
    ///
    /// Default: None
    pub cookie: Option<CookieSerializeOptions>,

    /// The options that will be forwarded to the cors module
    ///
    /// Default: None
    pub cors: Option<CorsOptions>
}
impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            ping_timeout: 20_000,
            ping_interval: 25_000,
            upgrade_timeout: 10_000,
            max_buffer_size: 100_000,
            allow_request: None,
            transports: vec![TransportType::Polling, TransportType::WebSocket],
            allow_upgrades: true,
            per_message_deflate: None,
            http_compression: true,
            initial_packet: None,
            cookie: None,
            cors: None,
        }
    }
}
impl ServerOptions {
    fn new(
        ping_timeout: Option<u32>,
        ping_interval: Option<u32>,
        upgrade_timeout: Option<u32>,
        max_buffer_size: Option<u32>,
        allow_request: Option<dyn Fn(
            EngineRequest,
            dyn Fn(Result<(), String>),
        )>,
        transports: Option<Vec<TransportType>>,
        allow_upgrades: Option<bool>,
        per_message_deflate: Option<u32>,
        http_compression: Option<bool>,
        initial_packet: Option<Packet>,
        cookie: Option<CookieSerializeOptions>,
        cors: Option<CorsOptions>
    ) -> Self {
        Self {
        ping_timeout: ping_timeout.unwrap_or(20_000),
        ping_interval: ping_interval.unwrap_or(25_000),
        upgrade_timeout: upgrade_timeout.unwrap_or(10_000),
        max_buffer_size: max_buffer_size.unwrap_or(100_000),
        allow_request,
        transports: transports.unwrap_or(vec![TransportType::Polling, TransportType::WebSocket]),
        allow_upgrades: allow_upgrades.unwrap_or(true),
        per_message_deflate,
        http_compression: http_compression.unwrap_or(true),
        initial_packet,
        cookie,
        cors,
        }
    }
}

pub type MiddleWare = dyn Fn(
    EngineRequest,
    EngineResult,
    dyn Fn(Error)
);

pub trait ServerBaseOperations {
    fn init(){}
    fn upgrades_to(){}
}
pub struct ServerBase {
    opts: ServerOptions,
    clients: HashMap<String, Socket>,
    pub clients_count: usize,
    middlewares: Vec<MiddleWare>,
    event_manager: EventEmitter
}
impl EventHandler for ServerBase {
    fn event_names(&self) -> Vec<String> {
        self.event_manager.event_names()
    }

    fn add_listener(&mut self, event_name: &str, callback: Listener) -> Result<(), EventError> {
        self.event_manager.add_listener(event_name, callback)
    }

    fn remove_listener(&mut self, event_name: &str, callback: Listener) -> Result<(), EventError> {
        self.event_manager.remove_listener(event_name, callback)
    }

    fn remove_all_listeners(&mut self, event_name: &str) -> Result<(), EventError> {
        self.event_manager.remove_all_listeners(event_name)
    }

    fn on(&mut self, event_name: &str, callback: Listener) -> Result<(), EventError> {
        self.event_manager.on(event_name, callback)
    }

    fn off(&mut self, event_name: &str, callback: Listener) -> Result<(), EventError> {
        self.event_manager.off(event_name, callback)
    }

    fn emit(&self, event_name: &str, payload: EventPayload) {
        self.event_manager.emit(event_name, payload)
    }

    fn set_max_listeners(&mut self, max: usize) {
        self.event_manager.set_max_listeners(max)
    }

    fn max_listeners(&self) -> usize {
        self.event_manager.max_listeners()
    }

    fn listener_count(&self, event_name: &str) -> Result<usize, EventError> {
        self.event_manager.listener_count(event_name)
    }
}
impl ServerBase {
    pub fn new(options: Option<ServerOptions>) -> Self {
        let base = Self {
            opts: options.unwrap_or_default(),
            clients: HashMap::new(),
            clients_count: 0,
            middlewares: vec![],
            event_manager: EventEmitter::new()
        };

        if let Some(mut cookie) =  &base.opts.cookie {
            cookie.set_name("greeno".to_string())
                .set_path(Some("/".into()))
                .set_http_only(true)
                .set_same_site(Some(SameSiteSetting::Lax));
        }

        if let Some(cors) = &base.opts.cors {
            base.use_middleware(Box::new(CorsMiddleware::new(cors)));
        }

        if let Some(mut deflate) = &base.opts.per_message_deflate {
            deflate = 1_024u32;
        }
        base
    }
    fn compute_path(options: &AttachOptions) -> String {
        let mut path = &options.path;
        // Remove trailing slash if present
        if path.ends_with('/') {
            path.pop();
        }
        // Add trailing slash unless explicitly disabled
        if options.add_trailing_slash {
            path.push('/');
        }

        path.to_string()
    }


    fn use_middleware(&self, func: Box<MiddleWare>){}
}

