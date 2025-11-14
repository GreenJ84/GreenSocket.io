use crate::{PacketError, PacketType};


#[test]
fn packet_type_from_str() {
  let mut packet_type = PacketType::try_from("open");
  assert_eq!(packet_type, Ok(PacketType::Open));

  packet_type = PacketType::try_from("close");
  assert_eq!(packet_type, Ok(PacketType::Close));

  packet_type = PacketType::try_from("ping");
  assert_eq!(packet_type, Ok(PacketType::Ping));

  packet_type = PacketType::try_from("pong"); 
  assert_eq!(packet_type, Ok(PacketType::Pong));

  packet_type = PacketType::try_from("message");
  assert_eq!(packet_type, Ok(PacketType::Message));

  packet_type = PacketType::try_from("upgrade");
  assert_eq!(packet_type, Ok(PacketType::Upgrade));

  packet_type = PacketType::try_from("invalid");
  assert_eq!(packet_type, Err(PacketError::InvalidPacketType));

  packet_type = PacketType::try_from("noop");
  assert_eq!(packet_type, Ok(PacketType::Noop));

  packet_type = PacketType::try_from("error");
  assert_eq!(packet_type, Ok(PacketType::Error));

  packet_type = PacketType::try_from("invalid");
  assert_eq!(packet_type, Err(PacketError::InvalidPacketType));
}

    #[test]
fn packet_type_from_char() {
  let mut packet_type = PacketType::try_from('0');
  assert_eq!(packet_type, Ok(PacketType::Open));

  packet_type = PacketType::try_from('1');
  assert_eq!(packet_type, Ok(PacketType::Close));

  packet_type = PacketType::try_from('2');
  assert_eq!(packet_type, Ok(PacketType::Ping));

  packet_type = PacketType::try_from('3');
  assert_eq!(packet_type, Ok(PacketType::Pong));

  packet_type = PacketType::try_from('4');
  assert_eq!(packet_type, Ok(PacketType::Message));

  packet_type = PacketType::try_from('5');
  assert_eq!(packet_type, Ok(PacketType::Upgrade));

  packet_type = PacketType::try_from('6');
  assert_eq!(packet_type, Ok(PacketType::Noop));

  packet_type = PacketType::try_from('9');
  assert_eq!(packet_type, Ok(PacketType::Error));

  packet_type = PacketType::try_from('8');
  assert_eq!(packet_type, Err(PacketError::InvalidPacketType));

  packet_type = PacketType::try_from('7');
  assert_eq!(packet_type, Err(PacketError::InvalidPacketType));

  packet_type = PacketType::try_from('!');
  assert_eq!(packet_type, Err(PacketError::InvalidPacketType));

  packet_type = PacketType::try_from('l');
  assert_eq!(packet_type, Err(PacketError::InvalidPacketType));
}

    #[test]
fn packet_type_from_u8() {
  let mut packet_type = PacketType::try_from(0);
  assert_eq!(packet_type, Ok(PacketType::Open));

  packet_type = PacketType::try_from(1);
  assert_eq!(packet_type, Ok(PacketType::Close));

  packet_type = PacketType::try_from(2);
  assert_eq!(packet_type, Ok(PacketType::Ping));

  packet_type = PacketType::try_from(3);
  assert_eq!(packet_type, Ok(PacketType::Pong));

  packet_type = PacketType::try_from(4);
  assert_eq!(packet_type, Ok(PacketType::Message));

  packet_type = PacketType::try_from(5);
  assert_eq!(packet_type, Ok(PacketType::Upgrade));

  packet_type = PacketType::try_from(6);
  assert_eq!(packet_type, Ok(PacketType::Noop));

  packet_type = PacketType::try_from(9);
  assert_eq!(packet_type, Ok(PacketType::Error));

  packet_type = PacketType::try_from(10);
  assert_eq!(packet_type, Err(PacketError::InvalidPacketType));

  packet_type = PacketType::try_from(11);
  assert_eq!(packet_type, Err(PacketError::InvalidPacketType));

  packet_type = PacketType::try_from(12);
  assert_eq!(packet_type, Err(PacketError::InvalidPacketType));

  packet_type = PacketType::try_from(u8::MAX);
  assert_eq!(packet_type, Err(PacketError::InvalidPacketType));
}

#[test]
fn packet_type_as_str() {
  let mut as_str: &str = PacketType::Open.into();
  assert_eq!(as_str, "open");

  as_str = PacketType::Close.into();
  assert_eq!(as_str, "close");

  as_str = PacketType::Ping.into();
  assert_eq!(as_str, "ping");

  as_str = PacketType::Pong.into();
  assert_eq!(as_str, "pong");

  as_str = PacketType::Message.into();
  assert_eq!(as_str, "message");

  as_str = PacketType::Upgrade.into();
  assert_eq!(as_str, "upgrade");

  as_str = PacketType::Noop.into();
  assert_eq!(as_str, "noop");

  as_str = PacketType::Error.into();
  assert_eq!(as_str, "error");
}

#[test]
fn packet_type_as_char() {
  let mut as_char: char = PacketType::Open.into();
  assert_eq!(as_char, '0');

  as_char = PacketType::Close.into();
  assert_eq!(as_char, '1');

  as_char = PacketType::Ping.into();
  assert_eq!(as_char, '2');

  as_char = PacketType::Pong.into();
  assert_eq!(as_char, '3');

  as_char = PacketType::Message.into();
  assert_eq!(as_char, '4');

  as_char = PacketType::Upgrade.into();
  assert_eq!(as_char, '5');

  as_char = PacketType::Noop.into();
  assert_eq!(as_char, '6');

  as_char = PacketType::Error.into();
  assert_eq!(as_char, '9');
}

#[test]
fn packet_type_as_u8() {
  let mut as_u8: u8 = PacketType::Open.into();
  assert_eq!(as_u8, 0);

  as_u8 = PacketType::Close.into();
  assert_eq!(as_u8, 1);

  as_u8 = PacketType::Ping.into();
  assert_eq!(as_u8, 2);

  as_u8 = PacketType::Pong.into();
  assert_eq!(as_u8, 3);

  as_u8 = PacketType::Message.into();
  assert_eq!(as_u8, 4);

  as_u8 = PacketType::Upgrade.into();
  assert_eq!(as_u8, 5);

  as_u8 = PacketType::Noop.into();
  assert_eq!(as_u8, 6);

  as_u8 = PacketType::Error.into();
  assert_eq!(as_u8, 9);
}