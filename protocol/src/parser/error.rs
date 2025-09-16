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
    Packet(PacketError),
    /// Base64 decoding failed.
    Base64(DecodeError),
    /// Raw Data is missing required fields decoding fields.
    MissingField,
    /// Packet data is invalid or malformed.
    InvalidFormat,
    /// Packet decoding failed without specific underlying error.
    UnknownError,
    /// Payload prefix length does not match actual data, or data is missing/extra.
    PayloadDataMismatch,
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodingError::Packet(e) => write!(f, "Packet decoding failed: {}", e),
            DecodingError::Base64(e) => write!(f, "Base64 decoding failed: {}", e),
            DecodingError::MissingField => write!(f, "Packet is missing required fields"),
            DecodingError::InvalidFormat => write!(f, "Packet data is invalid or malformed"),
            DecodingError::UnknownError => write!(f, "Unknown decoding error"),
            DecodingError::PayloadDataMismatch => write!(f, "Payload length prefix does not match actual data"),
        }
    }
}