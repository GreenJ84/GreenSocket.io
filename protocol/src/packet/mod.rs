pub(crate) mod options;
pub(crate) mod types;
pub(crate) mod error;

use options::PacketOptions;
use types::PacketType;
use error::PacketError;

use crate::constants::RawData;

/// Maximum allowed packet size (1 MB).
pub const MAX_PACKET_SIZE: usize = 1024 * 1024;

/// Represents a protocol packet, including its type, options, and data.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Packet {
    /// The type of the packet.
    pub _type: PacketType,
    /// Optional transmission options.
    pub options: Option<PacketOptions>,
    /// Optional packet data (text or binary).
    pub data: Option<RawData>,
}

impl Packet {
    /// Creates a new packet.
    pub fn new(_type: PacketType) -> Self {
        Self {
            _type,
            options: None,
            data: None
        }
    }

    /// Sets the packet options.
    pub fn with_options(mut self, options: PacketOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Sets the packet data.
    pub fn with_data(mut self, data: RawData) -> Result<(), PacketError> {
        if data.len() > MAX_PACKET_SIZE {
            return Err(PacketError::DataTooLarge);
        }
        self.data = Some(data);
        Ok(())
    }

    /// Creates an error packet with the given message.
    pub fn error(message: &str) -> Self {
        Self {
            _type: PacketType::Error,
            options: None,
            data: Some(RawData::Text(message.to_string())),
        }
    }
}