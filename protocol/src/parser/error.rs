use std::fmt;
use base64::DecodeError;

use crate::packet::error::PacketError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodingError {
    /// Packet encoding failed.
    EncodingError,
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodingError::EncodingError => write!(f, "Packet encoding failed"),
        }
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodingError {
    /// Packet decoding failed, with underlying packet error.
    DecodingError(PacketError),
    /// Base64 decoding failed.
    Base64DecodingError(DecodeError),
    /// Packet decoding failed without specific underlying error.
    UnknownDecodingError,
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodingError::DecodingError(e) => write!(f, "Packet decoding failed: {}", e),
            DecodingError::Base64DecodingError(e) => write!(f, "Base64 decoding failed: {}", e),
            DecodingError::UnknownDecodingError => write!(f, "Unknown decoding error"),
        }
    }
}