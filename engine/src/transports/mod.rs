pub(crate) mod transport;
pub(crate) mod polling;
pub(crate) mod polling_json;
pub(crate) mod web_socket;
pub(crate) mod web_transport;

use std::ops::Deref;

pub enum TransportType {
    Polling,
    WebSocket,
    WebTransport
}
pub fn polling_upgrades_to() -> Vec<TransportType> {
    vec![TransportType::WebSocket, TransportType::WebTransport]
}
