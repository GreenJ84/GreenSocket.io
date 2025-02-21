use std::ops::Deref;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use hyper::{Request, Response, Result};

use engine_parser::*;

fn noop(){}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ReadyState { Open, Closing, #[default] Closed }

pub type EngineRequest = Request<RawData>;
pub type EngineResponse = Response<RawData>;
pub type EngineResult = Result<EngineResponse>;

#[async_trait]
pub trait Transport: Send + Sync + EventHandler{
    // Abstract Methods
    fn get_name(&self) -> &str;

    fn on_request(&self, _req: EngineRequest);
    fn on_data(&self, data: RawData);
    fn on_packet(&self, packet: Packet);
    fn on_error(&self, msg: &str, desc: Option<&str>);
    async fn send(&self, packets: Vec<Packet>);

    fn on_close(&mut self);
    async fn do_close(&self);
    fn discard(&mut self);

}

///   properties:
///     - pub sid: String,
///     - pub writable: bool,
///     - pub event_manager: EventEmitter,
#[derive(Debug, Default)]
pub struct TransportBase {
    pub sid: String,
    pub writable: bool,
    pub event_manager: EventEmitter,
    ready_state: Arc<Mutex<ReadyState>>,
    discarded: bool,
    binary_supported: bool,
}
impl TransportBase {
    pub fn new(req: &EngineRequest) -> Self {
        let uri = req.uri();
        Self {
            // todo -- Add ID creation technique
            sid: String::new(),
            writable: false,
            event_manager: EventEmitter::new(),
            ready_state: Arc::new(Mutex::new(ReadyState::Open)),
            discarded: false,
            binary_supported: if let Some(q) = uri.query() {
                let mut b64_found_and_accepting: bool = false;
                let pairs = q.split("&");
                for pair in pairs {
                    let key_value = pair.split("=").collect::<Vec<&str>>();
                    if key_value[0] == "b64" && key_value[1] == "true" {
                        b64_found_and_accepting = true;
                    }
                }
                b64_found_and_accepting
            } else { false }
        }
    }

    pub fn ready_state(&self) -> ReadyState{
        self.ready_state.deref().unwrap()
    }
    pub fn set_ready_state(&mut self, state: ReadyState){
        *self.ready_state.get_mut().unwrap() = state;
    }

    pub fn discard(&mut self) {
        self.discarded = true;
    }
    pub fn is_discarded(&self) -> bool{
        self.discarded
    }

    pub fn supports_binary(&self) -> bool{
        self.discarded
    }

    pub fn close(&mut self) {
        let mut state = self.ready_state.lock().unwrap();
        if *state == ReadyState::Closed || *state == ReadyState::Closing {
            return;
        }
        *state = ReadyState::Closing;
    }
}