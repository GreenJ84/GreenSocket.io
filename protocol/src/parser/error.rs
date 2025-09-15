use crate::packet::error::PacketError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodingError {
    /// Packet encoding failed.
    EncodingError,
    /// Base64 encoding failed.
    Base64EncodingError,
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodingError::EncodingError => write!(f, "Packet encoding failed"),
            EncodingError::Base64EncodingError => write!(f, "Base64 encoding failed"),
        }
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodingError {
    /// Packet decoding failed, with underlying packet error.
    DecodingError(PacketError),
    /// Base64 decoding failed.
    Base64DecodingError,
    /// Packet decoding failed without specific underlying error.
    UnknownDecodingError,
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodingError::DecodingError(e) => write!(f, "Packet decoding failed: {}", e),
            DecodingError::Base64DecodingError => write!(f, "Base64 decoding failed"),
            DecodingError::UnknownDecodingError => write!(f, "Unknown decoding error"),
        }
    }
}