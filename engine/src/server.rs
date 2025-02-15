use http::{Error};

use engine_parser::Packet;
use crate::cookie::CookieSerializeOptions;
use crate::cors::CorsOptions;
use crate::transports::transport::{EngineRequest, EngineResult, Transport};

pub enum TransportType {
    Polling,
    WebSocket,
    WebTransport
}

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

pub struct ServerOptions {
    /// How many ms without a pong packet to consider the connection closed
    ///
    /// Default: 20_000
    ping_timeout: u32,

    ///how many ms before sending a new ping packet
    ///
    ///Default: 25_000
    ping_interval: u32,

    /// How many ms before an uncompleted transport upgrade is cancelled
    ///
    /// Default: 10_000
    upgrade_timeout: u32,

    /// How many bytes or characters a message can be, before closing the session (to avoid DoS).
    ///
    /// Default: 1e5 (100 KB)
    max_buffer_size: u32,

    /// A function that receives a given handshake or upgrade request as its first parameter,
    /// and can decide whether to continue or not. The second argument is a function that needs
    /// to be called with the decided information: fn(err, success), where success is a boolean
    /// value where false means that the request is rejected, and err is an error code.
    allow_request: Option<dyn Fn(
        EngineRequest,
        dyn Fn(Result<(), String>),
    )>,

    ///The low-level transports that are enabled. WebTransport is disabled by default and must be manually enabled:
    ///
    /// Default: `vec![TransportType::Polling, TransportType::WebSocket]`
    transports: Vec<TransportType>,

    /// Whether to allow transport upgrades
    ///
    /// Default: true
    allow_upgrades: bool,

    /// Enable WebSocket per-message-deflate extension
    ///
    /// Default: false
    per_message_deflate: bool,

    /// Enable http compression for the polling transports
    ///
    /// Default: true
    http_compression: bool,

    /// An optional packet which will be concatenated to the handshake packet emitted by Engine.IO.
    ///
    /// Default: None
    initial_packet: Option<Packet>,

    /// Configuration of the cookie that contains the client sid to send as part of handshake response headers. This cookie
    /// might be used for sticky-session. Defaults to not sending any cookie.
    ///
    /// Default: None
    cookie: Option<(CookieSerializeOptions, String)>,

    /// The options that will be forwarded to the cors module
    ///
    /// Default: None
    cors: Option<CorsOptions>
}

type MiddleWare = dyn Fn(
    EngineRequest,
    EngineResult,
    dyn Fn(Error)
);

