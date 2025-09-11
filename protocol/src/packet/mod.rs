pub(crate) mod encode;
pub(crate) mod decode;
pub(crate) mod encoding_stream;
pub(crate) mod decoding_stream;

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
    /// Creates a new packet..
    pub fn new(
        _type: PacketType,
        options: Option<PacketOptions>,
        data: Option<RawData>
    ) -> Result<Self, PacketError> {
        if let Some(ref d) = data {
            if d.len() > MAX_PACKET_SIZE {
                return Err(PacketError::DataTooLarge);
            }
        }
        Ok(Self { _type, options, data })
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