mod transport;

pub mod polling;
pub mod web_socket;
pub mod web_transport;

use std::ops::Deref;

pub use transport::{
    Transport,
    TransportBase,
    ReadyState,
    EngineRequest,
    EngineResponse,
    EngineResult,
};


#[derive(Clone, Debug, Default)]
pub enum TransportType {
    #[default]
    Polling,
    WebSocket,
    WebTransport
}

pub fn polling_upgrades_to() -> Vec<TransportType> {
    vec![TransportType::WebSocket, TransportType::WebTransport]
}
