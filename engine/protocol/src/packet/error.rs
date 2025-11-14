/// Error type for packet creation and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketError {
    /// Packet data exceeds the maximum allowed size.
    DataTooLarge,
    /// Packet type is invalid or unknown.
    InvalidPacketType,
    /// Packet options are invalid.
    InvalidPacketOptions,
    /// Chunking parameters are invalid.
    InvalidChunkingParameters,
}


impl std::error::Error for PacketError {}
impl std::fmt::Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketError::DataTooLarge => write!(f, "Packet data exceeds maximum allowed size"),
            PacketError::InvalidPacketType => write!(f, "Packet type is invalid or unknown"),
            PacketError::InvalidPacketOptions => write!(f, "Packet options are invalid or inconsistent"),
            PacketError::InvalidChunkingParameters => write!(f, "Invalid chunking parameters"),
        }
    }
}


