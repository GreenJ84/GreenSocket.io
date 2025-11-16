use std::convert::TryFrom;

use crate::protocol::PacketError;

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
    Error = 9,
}

impl TryFrom<&str> for PacketType {
  type Error = PacketError;

  fn try_from(s: &str) -> Result<Self, PacketError> {
    match s {
      "open" => Ok(Self::Open),
      "close" => Ok(Self::Close),
      "ping" => Ok(Self::Ping),
      "pong" => Ok(Self::Pong),
      "message" => Ok(Self::Message),
      "upgrade" => Ok(Self::Upgrade),
      "noop" => Ok(Self::Noop),
      "error" => Ok(Self::Error),
      _ => Err(PacketError::InvalidPacketType),
    }
  }
}

impl TryFrom<char> for PacketType {
  type Error = PacketError;

  fn try_from(c: char) -> Result<Self, PacketError> {
    match c {
      '0' => Ok(Self::Open),
      '1' => Ok(Self::Close),
      '2' => Ok(Self::Ping),
      '3' => Ok(Self::Pong),
      '4' => Ok(Self::Message),
      '5' => Ok(Self::Upgrade),
      '6' => Ok(Self::Noop),
      '9' => Ok(Self::Error),
      _ => Err(PacketError::InvalidPacketType),
    }
  }
}

impl TryFrom<u8> for PacketType {
  type Error = PacketError;

  fn try_from(c: u8) -> Result<Self, PacketError> {
    match c {
      0 => Ok(Self::Open),
      1 => Ok(Self::Close),
      2 => Ok(Self::Ping),
      3 => Ok(Self::Pong),
      4 => Ok(Self::Message),
      5 => Ok(Self::Upgrade),
      6 => Ok(Self::Noop),
      9 => Ok(Self::Error),
      _ => Err(PacketError::InvalidPacketType),
    }
  }
}

impl From<PacketType> for u8 {
    fn from(pt: PacketType) -> Self {
      match pt {
        PacketType::Open => 0,
        PacketType::Close => 1,
        PacketType::Ping => 2,
        PacketType::Pong => 3,
        PacketType::Message => 4,
        PacketType::Upgrade => 5,
        PacketType::Noop => 6,
        PacketType::Error => 9,
      }
    }
}

impl From<PacketType> for char {
    fn from(pt: PacketType) -> Self {
      match pt {
        PacketType::Open => '0',
        PacketType::Close => '1',
        PacketType::Ping => '2',
        PacketType::Pong => '3',
        PacketType::Message => '4',
        PacketType::Upgrade => '5',
        PacketType::Noop => '6',
        PacketType::Error => '9',
      }
    }
}

impl From<PacketType> for &'static str {
  fn from(pt: PacketType) -> Self {
    match pt {
      PacketType::Open => "open",
      PacketType::Close => "close",
      PacketType::Ping => "ping",
      PacketType::Pong => "pong",
      PacketType::Message => "message",
      PacketType::Upgrade => "upgrade",
      PacketType::Noop => "noop",
      PacketType::Error => "error",
    }
  }
}