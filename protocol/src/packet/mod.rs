pub(crate) mod options;
pub(crate) mod types;
pub(crate) mod error;

use super::constants::RawData;
use options::PacketOptions;
use types::PacketType;
use error::PacketError;


/// Maximum allowed packet size (1 MB).
pub const MAX_PACKET_SIZE: usize = 1024 * 1024;

/// Represents a protocol packet, including its type, options, and data.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Packet {
    /// The type of the packet.
    _type: PacketType,
    /// Optional transmission options.
    options: Option<PacketOptions>,
    /// Optional packet data (text or binary).
    data: Option<RawData>,
}

impl Packet {

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

    /// Returns the packet type.
    pub fn _type(&self) -> &PacketType {
        &self._type
    }

    /// Returns a reference to the packet options, if any.
    pub fn options(&self) -> Option<&PacketOptions> {
        self.options.as_ref()
    }

    /// Sets the packet options.
    pub fn with_options(mut self, options: PacketOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Returns a reference to the packet data, if any.
    pub fn data(&self) -> Option<&RawData> {
        self.data.as_ref()
    }

    /// Sets the packet data.
    pub fn with_data(&mut self, data: RawData) -> Result<(), PacketError> {
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