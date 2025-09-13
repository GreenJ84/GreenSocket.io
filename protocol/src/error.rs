/// Error type for packet creation and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolError {
    /// Packet data exceeds the maximum allowed size.
    DataTooLarge,
    /// Packet type is invalid or unknown.
    InvalidType,
    /// Packet options are invalid or inconsistent.
    InvalidOptions,
    /// Packet encoding failed.
    EncodeError,
    /// Packet decoding failed.
    DecodeError,
    /// Packet is missing required fields.
    MissingField,
}

impl std::error::Error for ProtocolError {}
impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::DataTooLarge => write!(f, "Packet data exceeds maximum allowed size"),
            ProtocolError::InvalidType => write!(f, "Packet type is invalid or unknown"),
            ProtocolError::InvalidOptions => write!(f, "Packet options are invalid or inconsistent"),
            ProtocolError::EncodeError => write!(f, "Packet encoding failed"),
            ProtocolError::DecodeError => write!(f, "Packet decoding failed"),
            ProtocolError::MissingField => write!(f, "Packet is missing required fields"),
        }
    }
}


