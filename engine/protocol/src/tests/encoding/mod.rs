#[cfg(test)]
mod options;

use base64::{Engine as _, engine::general_purpose};
use crate::{
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
fn non_data_or_option_binary_packets() {
    for pt in packet_type_iter() {
        let packet = Packet::new(pt.clone());
        let encoded = packet.encode(true);

        assert_eq!(encoded, RawData::Binary(vec![pt as u8, 0, 0]));
    }
}

#[test]
fn non_data_or_option_text_packets() {
    for pt in packet_type_iter() {
        let packet = Packet::new(pt.clone());
        let encoded = packet.clone().encode(false);

        assert_eq!(encoded, RawData::Text(format!("{}00", char::from(pt))));
    }
}

#[test]
fn packet_with_options_no_data_binary() {
    let mut packet = Packet::new(PacketType::Message);
    let opts = PacketOptions::default().with_compression().with_encryption();
    packet.with_options(opts);

    let encoded = packet.encode(true);
    assert_eq!(
        encoded,
        RawData::Binary(vec![PacketType::Message as u8, 1, 0, 1, 1, 0, 0, 0, 0])
    );
}

#[test]
fn packet_with_options_no_data_text() {
    let mut packet = Packet::new(PacketType::Message);
    let opts = PacketOptions::default().with_compression().with_encryption();
    packet.with_options(opts);

    let encoded = packet.encode(false);
    assert_eq!(
        encoded,
        RawData::Text(format!("{}101:1:0:0", char::from(PacketType::Message)))
    );
}



fn small_data_packet(binary: bool) -> Packet {
    let mut packet = Packet::new(PacketType::Message);
    let data = match binary {
        true => RawData::Binary(vec![1, 2, 3]),
        false => RawData::Text("abc".to_string()),
    };
    packet.with_data(data.clone()).unwrap();
    packet
}

#[test]
fn packet_with_small_binary_data() {
    let packet = small_data_packet(true);
    let encoded = packet.encode(true);
    // [type, 0, 1, 1, 2, 3]
    assert_eq!(
        encoded,
        RawData::Binary(vec![PacketType::Message as u8, 0, 1, BINARY_MASK, 1, 2, 3])
    );
}

#[test]
fn packet_with_small_binary_data_cross_encoding() {
    let packet = small_data_packet(true);
    // Text encoding (should be base64)
    let encoded_text = packet.encode(false);

    let base64 = general_purpose::URL_SAFE.encode(&[1, 2, 3]);
    assert_eq!(
        encoded_text,
        RawData::Text(format!("{}01-b{}", char::from(PacketType::Message), base64))
    );
}

#[test]
fn packet_with_small_text_data() {
    let packet = small_data_packet(false);
    let encoded = packet.encode(false);

    assert_eq!(
        encoded,
        RawData::Text(format!("{}01-tabc", char::from(PacketType::Message)))
    );
}

#[test]
fn packet_with_small_text_data_cross_encoding() {
    let packet = small_data_packet(false);
    // Binary encoding (should be plain bytes)
    let encoded_bin = packet.encode(true);

    let mut expected = vec![PacketType::Message as u8, 0, 1, PLAIN_TEXT_MASK];
    expected.extend(b"abc");
    assert_eq!(encoded_bin, RawData::Binary(expected));
}



fn large_data_packet(binary: bool) -> Packet {
    let mut packet = Packet::new(PacketType::Message);
    let data = match binary {
        true => RawData::Binary(vec![42; 1024]),
        false => RawData::Text("x".repeat(1024)),
    };
    packet.with_data(data.clone()).unwrap();
    packet
}

#[test]
fn packet_with_large_binary_data() {
    let packet = large_data_packet(true);
    let encoded = packet.encode(true);

    let mut expected = vec![PacketType::Message as u8, 0, 1];
    expected.push(BINARY_MASK);
    expected.extend(vec![42; 1024]);

    assert_eq!(encoded, RawData::Binary(expected));
}

#[test]
fn packet_with_large_binary_data_cross_encoding() {
    let packet = large_data_packet(true);
    // Text encoding (should be base64)
    let encoded_text = packet.encode(false);

    let base64 = general_purpose::URL_SAFE.encode(&vec![42; 1024]);
    assert_eq!(
        encoded_text,
        RawData::Text(format!("{}01-b{}", char::from(PacketType::Message), base64))
    );
}

#[test]
fn packet_with_large_text_data() {
    let packet = large_data_packet(false);
    let encoded = packet.encode(false);

    assert_eq!(
        encoded,
        RawData::Text(format!("{}01-t{}", char::from(PacketType::Message), "x".repeat(1024)))
    );
}

#[test]
fn packet_with_large_text_data_cross_encoding() {
    let packet = large_data_packet(false);
    // Binary encoding (should be plain bytes)
    let encoded_bin = packet.encode(true);
    let mut expected_bin = vec![PacketType::Message as u8, 0, 1, PLAIN_TEXT_MASK];
    expected_bin.extend("x".repeat(1024).as_bytes());
    assert_eq!(encoded_bin, RawData::Binary(expected_bin));
}




fn packet_with_options_and_data(binary: bool) -> Packet {
    let mut packet = Packet::new(PacketType::Message);

    let mut opts = PacketOptions::default().with_compression();
    opts.with_chunking(2, 4).ok();
    packet.with_options(opts);

    let data = match binary {
        true => RawData::Binary(vec![9, 8, 7]),
        false => RawData::Text("xyz".to_string()),
    };
    packet.with_data(data.clone()).ok();

    packet
}

#[test]
fn packet_with_options_and_data_binary() {
    let packet = packet_with_options_and_data(true);

    let encoded = packet.encode(true);
    // [type, 1, 0, 0, 2, 0, 4, BINARY_MASK, 9, 8, 7]
    assert_eq!(
        encoded,
        RawData::Binary(vec![PacketType::Message as u8, 1, 1, 1, 0, 0, 2, 0, 4, BINARY_MASK, 9, 8, 7])
    );
}

#[test]
fn packet_with_options_and_data_binary_cross_encoding() {
    let packet = packet_with_options_and_data(true);
    // Text encoding (should be base64)
    let encoded_text = packet.encode(false);

    let base64 = general_purpose::URL_SAFE.encode(&[9, 8, 7]);
    assert_eq!(
        encoded_text,
        RawData::Text(format!("{}111:0:2:4-b{}", char::from(PacketType::Message), base64))
    );
}

#[test]
fn packet_with_options_and_data_text() {
    let packet = packet_with_options_and_data(false);

    let encoded = packet.encode(false);
    assert_eq!(
        encoded,
        RawData::Text(format!("{}111:0:2:4-txyz", char::from(PacketType::Message)))
    );
}

#[test]
fn packet_with_options_and_data_text_cross_encoding() {
    let packet = packet_with_options_and_data(false);
    // Binary encoding (should be plain bytes)
    let encoded_bin = packet.encode(true);

    let mut expected_bin = vec![PacketType::Message as u8, 1, 1, 1, 0, 0, 2, 0, 4, PLAIN_TEXT_MASK];
    expected_bin.extend(b"xyz");
    assert_eq!(encoded_bin, RawData::Binary(expected_bin));
}

#[test]
fn packet_over_data_limit_binary() {
    let mut packet = Packet::new(PacketType::Message);
    let data = RawData::Binary(vec![0; MAX_PACKET_SIZE + 1]);
    let result = packet.with_data(data);
    assert!(result.is_err());
}

#[test]
fn packet_over_data_limit_text() {
    let mut packet = Packet::new(PacketType::Message);
    let data = RawData::Text("a".repeat(MAX_PACKET_SIZE + 1));
    let result = packet.with_data(data);
    assert!(result.is_err());
}
