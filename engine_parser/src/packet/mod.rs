pub(crate) mod encode;
pub(crate) mod decode;
pub(crate) mod encoding_stream;
pub(crate) mod decoding_stream;

use crate::constants::RawData;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketType {
    Open = 0,
    Close = 1,
    Ping = 2,
    Pong = 3,
    Message = 4,
    Upgrade = 5,
    Noop = 6,
    Error = -1,
}
impl PacketType {
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
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(Self::Open),
            '1' => Some(Self::Close),
            '2' => Some(Self::Ping),
            '3' => Some(Self::Pong),
            '4' => Some(Self::Message),
            '5' => Some(Self::Upgrade),
            '6' => Some(Self::Noop),
            'e' => Some(Self::Error), // Error mapped to "-1" in your previous implementation
            _ => None,
        }
    }
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
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PacketOptions {
    pub compress: bool,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Packet {
    pub _type: PacketType,
    pub options: Option<PacketOptions>,
    pub data: Option<RawData>,
}
impl Packet {
    pub fn new(
        _type: PacketType,
        options: Option<PacketOptions>,
        data: Option<RawData>
    ) -> Self{
        return Self { _type, options, data };
    }

    pub fn error(message: &str) -> Self {
        Self {
            _type: PacketType::Error,
            options: None,
            data: Some(RawData::Binary(message.bytes().collect())),
        }
    }
}

