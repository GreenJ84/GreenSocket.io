#[cfg(test)]
mod options;

use base64::{Engine as _, engine::general_purpose};
use crate::protocol::{
    Packet,
    PacketOptions,
    PacketType,
    RawData,
    MAX_PACKET_SIZE,
    constants::{BINARY_MASK, PLAIN_TEXT_MASK},
};

fn packet_type_iter() -> impl Iterator<Item = PacketType> {
    (0u8..=9)
        .filter_map(|v| PacketType::try_from(v).ok())
}

#[test]
fn decode_non_data_or_option_encoded_binary() {
    for pt in packet_type_iter() {
        let encoded = RawData::Binary(vec![pt.clone() as u8, 0, 0]);
        let decoded = Packet::decode(encoded.clone()).unwrap();
        let expected = Packet::new(pt);
        assert_eq!(decoded, expected);
    }
}

#[test]
fn decode_non_data_or_option_encoded_text() {
    for pt in packet_type_iter() {
        let encoded = RawData::Text(format!("{}00", char::from(pt.clone())));
        let decoded = Packet::decode(encoded.clone()).unwrap();
        let expected = Packet::new(pt);
        assert_eq!(decoded, expected);
    }
}

#[test]
fn decode_packet_with_options_no_data_binary() {
    let encoded = RawData::Binary(vec![PacketType::Message as u8, 1, 0, 1, 1, 0, 0, 0, 0]);
    let decoded = Packet::decode(encoded.clone()).unwrap();
    let mut expected = Packet::new(PacketType::Message);
    let opts = PacketOptions::default().with_compression().with_encryption();
    expected.with_options(opts);
    assert_eq!(decoded, expected);
}

#[test]
fn decode_packet_with_options_no_data_text() {
    let encoded = RawData::Text(format!("{}101:1:0:0", char::from(PacketType::Message)));
    let decoded = Packet::decode(encoded.clone()).unwrap();
    let mut expected = Packet::new(PacketType::Message);
    let opts = PacketOptions::default().with_compression().with_encryption();
    expected.with_options(opts);
    assert_eq!(decoded, expected);
}

fn small_data_packet(binary: bool) -> (Packet, RawData) {
    let mut packet = Packet::new(PacketType::Message);
    let data = match binary {
        true => RawData::Binary(vec![1, 2, 3]),
        false => RawData::Text("abc".to_string()),
    };
    packet.with_data(data.clone()).unwrap();
    (packet, data)
}

#[test]
fn decode_packet_with_small_binary_data() {
    let encoded = RawData::Binary(vec![PacketType::Message as u8, 0, 1, BINARY_MASK, 1, 2, 3]);
    let decoded = Packet::decode(encoded.clone()).unwrap();
    let (expected, _) = small_data_packet(true);
    assert_eq!(decoded, expected);
}

#[test]
fn decode_packet_with_small_binary_data_cross_encoding() {
    let base64 = general_purpose::URL_SAFE.encode(&[1, 2, 3]);
    let encoded = RawData::Text(format!("{}01-b{}", char::from(PacketType::Message), base64));
    let decoded = Packet::decode(encoded.clone()).unwrap();
    let (expected, _) = small_data_packet(true);
    assert_eq!(decoded, expected);
}

#[test]
fn decode_packet_with_small_text_data() {
    let encoded = RawData::Text(format!("{}01-tabc", char::from(PacketType::Message)));
    let decoded = Packet::decode(encoded.clone()).unwrap();
    let (expected, _) = small_data_packet(false);
    assert_eq!(decoded, expected);
}

#[test]
fn decode_packet_with_small_text_data_cross_encoding() {
    let mut expected = Packet::new(PacketType::Message);
    let data = RawData::Text("abc".to_string());
    expected.with_data(data.clone()).unwrap();
    let mut bin = vec![PacketType::Message as u8, 0, 1, PLAIN_TEXT_MASK];
    bin.extend(b"abc");
    let encoded = RawData::Binary(bin);
    let decoded = Packet::decode(encoded.clone()).unwrap();
    assert_eq!(decoded, expected);
}

fn large_data_packet(binary: bool) -> (Packet, RawData) {
    let mut packet = Packet::new(PacketType::Message);
    let data = match binary {
        true => RawData::Binary(vec![42; 1024]),
        false => RawData::Text("x".repeat(1024)),
    };
    packet.with_data(data.clone()).unwrap();
    (packet, data)
}

#[test]
fn decode_packet_with_large_binary_data() {
    let mut expected_bin = vec![PacketType::Message as u8, 0, 1];
    expected_bin.push(BINARY_MASK);
    expected_bin.extend(vec![42; 1024]);
    let encoded = RawData::Binary(expected_bin);
    let decoded = Packet::decode(encoded.clone()).unwrap();
    let (expected, _) = large_data_packet(true);
    assert_eq!(decoded, expected);
}

#[test]
fn decode_packet_with_large_binary_data_cross_encoding() {
    let base64 = general_purpose::URL_SAFE.encode(&vec![42; 1024]);
    let encoded = RawData::Text(format!("{}01-b{}", char::from(PacketType::Message), base64));
    let decoded = Packet::decode(encoded.clone()).unwrap();
    let (expected, _) = large_data_packet(true);
    assert_eq!(decoded, expected);
}

#[test]
fn decode_packet_with_large_text_data() {
    let encoded = RawData::Text(format!("{}01-t{}", char::from(PacketType::Message), "x".repeat(1024)));
    let decoded = Packet::decode(encoded.clone()).unwrap();
    let (expected, _) = large_data_packet(false);
    assert_eq!(decoded, expected);
}

#[test]
fn decode_packet_with_large_text_data_cross_encoding() {
    let mut expected = Packet::new(PacketType::Message);
    let data = RawData::Text("x".repeat(1024));
    expected.with_data(data.clone()).unwrap();
    let mut bin = vec![PacketType::Message as u8, 0, 1, PLAIN_TEXT_MASK];
    bin.extend("x".repeat(1024).as_bytes());
    let encoded = RawData::Binary(bin);
    let decoded = Packet::decode(encoded.clone()).unwrap();
    assert_eq!(decoded, expected);
}

fn packet_with_options_and_data(binary: bool) -> (Packet, RawData) {
    let mut packet = Packet::new(PacketType::Message);

    let mut opts = PacketOptions::default().with_compression();
    opts.with_chunking(2, 4).ok();
    packet.with_options(opts);

    let data = match binary {
        true => RawData::Binary(vec![9, 8, 7]),
        false => RawData::Text("xyz".to_string()),
    };
    packet.with_data(data.clone()).ok();

    (packet, data)
}

#[test]
fn decode_packet_with_options_and_data_binary() {
    let encoded = RawData::Binary(vec![PacketType::Message as u8, 1, 1, 1, 0, 0, 2, 0, 4, BINARY_MASK, 9, 8, 7]);
    let decoded = Packet::decode(encoded.clone()).unwrap();
    let (expected, _) = packet_with_options_and_data(true);
    assert_eq!(decoded, expected);
}

#[test]
fn decode_packet_with_options_and_data_binary_cross_encoding() {
    let base64 = general_purpose::URL_SAFE.encode(&[9, 8, 7]);
    let encoded = RawData::Text(format!("{}111:0:2:4-b{}", char::from(PacketType::Message), base64));
    let decoded = Packet::decode(encoded.clone()).unwrap();
    let (expected, _) = packet_with_options_and_data(true);
    assert_eq!(decoded, expected);
}

#[test]
fn decode_packet_with_options_and_data_text() {
    let encoded = RawData::Text(format!("{}111:0:2:4-txyz", char::from(PacketType::Message)));
    let decoded = Packet::decode(encoded.clone()).unwrap();
    let (expected, _) = packet_with_options_and_data(false);
    assert_eq!(decoded, expected);
}

#[test]
fn decode_packet_with_options_and_data_text_cross_encoding() {
    let mut expected = Packet::new(PacketType::Message);
    let mut opts = PacketOptions::default().with_compression();
    opts.with_chunking(2, 4).ok();
    expected.with_options(opts);
    let data = RawData::Text("xyz".to_string());
    expected.with_data(data.clone()).ok();
    let mut bin = vec![PacketType::Message as u8, 1, 1, 1, 0, 0, 2, 0, 4, PLAIN_TEXT_MASK];
    bin.extend(b"xyz");
    let encoded = RawData::Binary(bin);
    let decoded = Packet::decode(encoded.clone()).unwrap();
    assert_eq!(decoded, expected);
}

#[test]
fn decode_packet_over_data_limit_binary() {
    let mut bin = vec![PacketType::Message as u8, 0, 1, BINARY_MASK];
    bin.extend(vec![0; MAX_PACKET_SIZE + 1]);
    let encoded = RawData::Binary(bin);
    let result = Packet::decode(encoded);
    assert!(result.is_err());
}

#[test]
fn decode_packet_over_data_limit_text() {
    let encoded = RawData::Text(format!("{}01-t{}", char::from(PacketType::Message), "a".repeat(MAX_PACKET_SIZE + 1)));
    let result = Packet::decode(encoded);
    assert!(result.is_err());
}

