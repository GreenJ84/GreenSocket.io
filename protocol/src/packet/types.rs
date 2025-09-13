use std::convert::TryFrom;

use crate::error::ProtocolError;

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
    Error = 255,
}

impl TryFrom<&str> for PacketType {
  type Error = ProtocolError;

  fn try_from(s: &str) -> Result<Self, ProtocolError> {
    match s {
      "open" => Ok(Self::Open),
      "close" => Ok(Self::Close),
      "ping" => Ok(Self::Ping),
      "pong" => Ok(Self::Pong),
      "message" => Ok(Self::Message),
      "upgrade" => Ok(Self::Upgrade),
      "noop" => Ok(Self::Noop),
      "error" => Ok(Self::Error),
      _ => Err(ProtocolError::InvalidType),
    }
  }
}

impl TryFrom<u8> for PacketType {
  type Error = ProtocolError;

  fn try_from(c: u8) -> Result<Self, ProtocolError> {
    match c {
      0 => Ok(Self::Open),
      1 => Ok(Self::Close),
      2 => Ok(Self::Ping),
      3 => Ok(Self::Pong),
      4 => Ok(Self::Message),
      5 => Ok(Self::Upgrade),
      6 => Ok(Self::Noop),
      255 => Ok(Self::Error),
      _ => Err(ProtocolError::InvalidType),
    }
  }
}

impl PacketType {
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

  /// Returns the integer representation of the packet type.
  pub fn as_int(&self) -> u8 {
    match self {
      Self::Open => 0,
      Self::Close => 1,
      Self::Ping => 2,
      Self::Pong => 3,
      Self::Message => 4,
      Self::Upgrade => 5,
      Self::Noop => 6,
      Self::Error => 255,
    }
  }
}