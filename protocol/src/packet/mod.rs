pub(crate) mod encode;
pub(crate) mod decode;
pub(crate) mod encoding_stream;
pub(crate) mod decoding_stream;

use crate::constants::RawData;

/// Represents the type of packet.
/// Each variant corresponds to a specific packet type in the protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketType {
    /// Open Connection.
    Open = 0,
    /// Close Connection.
    Close = 1,
    /// Ping packet. (Heartbeat implementation)
    Ping = 2,
    /// Pong packet. (Heartbeat implementation)
    Pong = 3,
    /// Message packet.
    Message = 4,
    /// Transport upgrade packet.
    Upgrade = 5,
    /// No-operation packet.
    Noop = 6,
    /// Error packet.
    Error = -1,
}

impl PacketType {
    /// Converts a string to a `PacketType`, if possible.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "open" => Some(Self::Open),
            "close" => Some(Self::Close),
            "ping" => Some(Self::Ping),
            "pong" => Some(Self::Pong),
            "message" => Some(Self::Message),
            "upgrade" => Some(Self::Upgrade),
            "noop" => Some(Self::Noop),
            "error" => Some(Self::Error),
            _ => None,
        }
    }

    /// Converts a character to a `PacketType`, if possible.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(Self::Open),
            '1' => Some(Self::Close),
            '2' => Some(Self::Ping),
            '3' => Some(Self::Pong),
            '4' => Some(Self::Message),
            '5' => Some(Self::Upgrade),
            '6' => Some(Self::Noop),
            'e' => Some(Self::Error), // Error mapped to "-1"
            _ => None,
        }
    }

    /// Converts a integer to a `PacketType`, if possible.
    pub fn from_int(c: i8) -> Option<Self> {
        match c {
            0 => Some(Self::Open),
            1 => Some(Self::Close),
            2 => Some(Self::Ping),
            3 => Some(Self::Pong),
            4 => Some(Self::Message),
            5 => Some(Self::Upgrade),
            6 => Some(Self::Noop),
            -1 => Some(Self::Error),
            _ => None,
        }
    }

    /// Returns the string representation of the packet type.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Close => "close",
            Self::Ping => "ping",
            Self::Pong => "pong",
            Self::Message => "message",
            Self::Upgrade => "upgrade",
            Self::Noop => "noop",
            Self::Error => "error",
        }
    }

    /// Returns the character representation of the packet type.
    pub fn as_char(&self) -> char {
        match self {
            Self::Open => '0',
            Self::Close => '1',
            Self::Ping => '2',
            Self::Pong => '3',
            Self::Message => '4',
            Self::Upgrade => '5',
            Self::Noop => '6',
            Self::Error => 'e',
        }
    }

    /// Returns the integer representation of the packet type.
    pub fn as_int(&self) -> i8 {
        match self {
            Self::Open => 0,
            Self::Close => 1,
            Self::Ping => 2,
            Self::Pong => 3,
            Self::Message => 4,
            Self::Upgrade => 5,
            Self::Noop => 6,
            Self::Error => -1,
        }
    }
}

/// Maximum allowed packet size (1 MB).
pub const MAX_PACKET_SIZE: usize = 1024 * 1024;

/// Error type for packet creation and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketError {
    /// Packet data exceeds the maximum allowed size.
    DataTooLarge,
}



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
    ///
    /// # Arguments
    ///
    /// * `_type` - The type of the packet.
    /// * `options` - Optional transmission options.
    /// * `data` - Optional packet data (text or binary).
    ///
    /// # Errors
    ///
    ///! Returns `PacketError::DataTooLarge` if the data exceeds the maximum allowed size.
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
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to include in the packet.
    pub fn error(message: &str) -> Self {
        Self {
            _type: PacketType::Error,
            options: None,
            data: Some(RawData::Text(message.to_string())),
        }
    }
}



/// Options for packet transmission, such as compression.
// #[cfg(not(feature = "sequencing"))]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PacketOptions {
    /// Whether the packet should be compressed.
    pub compress: bool,
}

// #[cfg(feature = "sequencing")]
// #[derive(PartialEq, Eq, Debug, Clone)]
// pub struct PacketOptions {
//     pub compress: bool,
//     pub sequence: Option<usize>,
//     pub total_chunks: Option<usize>,
//