/// Error type for packet creation and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketError {
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


