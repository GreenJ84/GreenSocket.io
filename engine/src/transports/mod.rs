pub(crate) mod transport;
pub(crate) mod polling;
pub(crate) mod polling_json;
pub(crate) mod web_socket;
pub(crate) mod web_transport;
pub(crate) mod socket;

use std::ops::Deref;
use std::collections::HashMap;
use std::sync::Arc;

use crate::transports::transport::EngineRequest;

pub fn transport_map() -> HashMap<&'static str, Arc<dyn transport::Transport>> {
    let mut map: HashMap<&str, Arc<dyn transport::Transport>> = HashMap::new();
    map.insert("polling", Arc::new(polling::Polling));
    map.insert("json_polling", Arc::new(polling_json::JSONP));
    map.insert("websocket", Arc::new(web_socket::WebSocket));
    map.insert("webtransport", Arc::new(web_transport::WebTransport));
    map
}

pub fn polling(req: EngineRequest) -> Box<dyn transport::Transport> {
    let (parts, body) = req.into_parts();
    if parts.uri.query().get("j").is_some() {
        Box::new(polling_json::JSONP)
    } else {
        Box::new(polling::Polling)
    }
}


pub fn polling_upgrades_to() -> Vec<&'static str> {
    vec!["web_socket", "web_transport"]
}
