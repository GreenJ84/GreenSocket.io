#[cfg(test)]
mod types;

#[cfg(test)]
mod options;

use crate::protocol::{RawData, Packet, PacketError, PacketOptions, PacketType, MAX_PACKET_SIZE};

#[test]
fn valid_full_packet_creation() {
    let packet_type = PacketType::Message;
    let options = PacketOptions::default().with_compression();
    let data = RawData::Binary(vec![1, 2, 3, 4, 5]);

    let mut packet = Packet::new(packet_type.clone());
    packet.with_options(options);
    packet.with_data(data.clone()).unwrap();

    assert_eq!(packet._type(), &packet_type);
    assert_eq!(packet.options(), Some(&options));
    assert_eq!(packet.data(), Some(&data));
}

#[test]
fn valid_packet_creation_without_options() {
    let packet_type = PacketType::Message;
    let data = RawData::Text(String::from("Hello World!"));

    let mut packet = Packet::new(packet_type.clone());
    packet.with_data(data.clone()).unwrap();

    assert_eq!(packet._type(), &packet_type);
    assert!(packet.options().is_none());
    assert_eq!(packet.data(), Some(&data));
}

#[test]
fn valid_packet_creation_without_data() {
    let packet_type = PacketType::Message;
    let options = PacketOptions::default();

    let mut packet = Packet::new(packet_type.clone());
    packet.with_options(options.clone());

    assert_eq!(packet._type(), &PacketType::Message);
    assert_eq!(packet.options(), Some(&options));
    assert!(packet.data().is_none());
}

#[test]
fn valid_packet_creation_without_options_or_data() {
    let packet_type = PacketType::Ping;
    let packet = Packet::new(packet_type);

    assert_eq!(packet._type(), &PacketType::Ping);
    assert!(packet.options().is_none());
    assert!(packet.data().is_none());
}

#[test]
fn valid_packet_creation_large_data() {
    let large_data = RawData::Binary(vec![0; 10_000]);
    let mut packet = Packet::new(PacketType::Message);
    packet.with_data(large_data.clone()).unwrap();

    if let Some(RawData::Binary(bin)) = packet.data() {
        assert_eq!(bin.len(), 10_000);
    } else { panic!("Expected binary data") }
}

#[test]
fn error_packet_creation() {
    let message = "An error occurred";
    let packet = Packet::error(message);

    assert_eq!(packet._type(), &PacketType::Error);
    assert!(packet.options().is_none());
    assert_eq!(packet.data(), Some(&RawData::Text(message.to_string())));
}

#[test]
fn invalid_packet_creation_too_large() {
    let large_data = RawData::Binary(vec![0; MAX_PACKET_SIZE + 1]);
    let mut packet = Packet::new(PacketType::Message);
    let result = packet.with_data(large_data);
    assert!(result.is_err());
    assert!(matches!(result, Err(PacketError::DataTooLarge)));
}

#[test]
fn packet_setters_and_getters() {
    let mut packet = Packet::new(PacketType::Ping);
    let options = PacketOptions::default();
    packet.with_options(options.clone());
    assert_eq!(packet.options(), Some(&options));

    let data = RawData::Text("test".to_string());
    packet.with_data(data.clone()).unwrap();
    assert_eq!(packet.data(), Some(&data));
}

#[test]
fn packet_clone_validation() {
    let mut packet = Packet::new(PacketType::Message);
    packet.with_data(RawData::Binary(vec![1, 2, 3])).unwrap();

    let mut cloned_packet = packet.clone();

    assert_eq!(packet, cloned_packet);
    cloned_packet.with_options(PacketOptions::default().with_compression());
    assert_ne!(packet, cloned_packet);
}

#[test]
fn packet_partial_eq_validation() {
    let mut packet1 = Packet::new(PacketType::Message);
    packet1.with_options(PacketOptions::default());
    packet1.with_data(RawData::Text("Hello World".to_string())).unwrap();

    let mut packet2 = Packet::new(PacketType::Message);
    packet2.with_options(PacketOptions::default());
    packet2.with_data(RawData::Text("Hello World".to_string())).unwrap();

    let packet3 = Packet::new(PacketType::Ping);

    assert_eq!(packet1, packet2);
    assert_ne!(packet1, packet3);
}

// #[cfg(test)]
// mod packet_encoding_decoding_tests {
// use super::*;

// #[test]
// fn encode_decode_binary() {
//     let packet = Packet::new(
//         PacketType::Message,
//         Some(PacketOptions { compress: true, ..Default::default() }),
//         Some(RawData::Binary(vec![1, 2, 3, 4])),
//     ).unwrap();

//     let encoded = packet.clone().encode(true);
//     let decoded = Packet::decode_binary(encoded.into_binary().unwrap()).unwrap();

//     assert_eq!(packet, decoded);
// }

// #[test]
// fn encode_decode_text() {
//     let packet = Packet::new(
//         PacketType::Message,
//         Some(PacketOptions { compress: false, ..Default::default() }),
//         Some(RawData::Text("Hello".to_string())),
//     ).unwrap();

//     let encoded = packet.clone().encode(false);
//     let decoded = Packet::decode_text(encoded.into_text().unwrap()).unwrap();

//     assert_eq!(packet, decoded);
// }

// #[test]
// fn encode_decode_error_packet() {
//     let packet = Packet::error("fail");
//     let encoded = packet.clone().encode(false);
//     let decoded = Packet::decode_text(encoded.into_text().unwrap()).unwrap();
//     assert_eq!(packet, decoded);
// }

// #[test]
// fn encode_decode_large_binary() {
//     let data = vec![42u8; 1024];
//     let packet = Packet::new(
//         PacketType::Message,
//         None,
//         Some(RawData::Binary(data.clone())),
//     ).unwrap();

//     let encoded = packet.clone().encode(true);
//     let decoded = Packet::decode_binary(encoded.into_binary().unwrap()).unwrap();
//     assert_eq!(packet, decoded);
// }

