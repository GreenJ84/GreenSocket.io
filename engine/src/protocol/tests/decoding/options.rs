use crate::protocol::{DecodingError, PacketError, PacketOptions, RawData};

#[test]
fn decode_default_binary() {
    let raw = RawData::Binary(vec![0, 0, 0, 0, 0, 0]);
    let opts = PacketOptions::decode(raw).expect("should decode default binary");
    assert_eq!(opts, PacketOptions::default());
}

#[test]
fn decode_default_text() {
    let raw = RawData::Text("0:0:0:0".into());
    let opts = PacketOptions::decode(raw).expect("should decode default text");
    assert_eq!(opts, PacketOptions::default());
}

#[test]
fn decode_compress_binary() {
    let raw = RawData::Binary(vec![1, 0, 0, 0, 0, 0]);
    let opts = PacketOptions::decode(raw).expect("should decode compress binary");
    let mut expected = PacketOptions::default();
    expected = expected.with_compression();
    assert_eq!(opts, expected);
}

#[test]
fn decode_compress_text() {
    let raw = RawData::Text("1:0:0:0".into());
    let opts = PacketOptions::decode(raw).expect("should decode compress text");
    let mut expected = PacketOptions::default();
    expected = expected.with_compression();
    assert_eq!(opts, expected);
}

#[test]
fn decode_encrypt_binary() {
    let raw = RawData::Binary(vec![0, 1, 0, 0, 0, 0]);
    let opts = PacketOptions::decode(raw).expect("should decode encrypt binary");
    let mut expected = PacketOptions::default();
    expected = expected.with_encryption();
    assert_eq!(opts, expected);
}

#[test]
fn decode_encrypt_text() {
    let raw = RawData::Text("0:1:0:0".into());
    let opts = PacketOptions::decode(raw).expect("should decode encrypt text");
    let mut expected = PacketOptions::default();
    expected = expected.with_encryption();
    assert_eq!(opts, expected);
}

#[test]
fn decode_compress_and_encrypt_binary() {
    let raw = RawData::Binary(vec![1, 1, 0, 0, 0, 0]);
    let opts = PacketOptions::decode(raw).expect("should decode compress+encrypt binary");
    let mut expected = PacketOptions::default();
    expected = expected.with_compression().with_encryption();
    assert_eq!(opts, expected);
}

#[test]
fn decode_compress_and_encrypt_text() {
    let raw = RawData::Text("1:1:0:0".into());
    let opts = PacketOptions::decode(raw).expect("should decode compress+encrypt text");
    let mut expected = PacketOptions::default();
    expected = expected.with_compression().with_encryption();
    assert_eq!(opts, expected);
}

#[test]
fn decode_chunk_data_binary() {
    // sequence = 69 (0x0045), total = 4321 (0x10E1)
    let raw = RawData::Binary(vec![0, 0, 0, 69, 16, 225]);
    let opts = PacketOptions::decode(raw).expect("should decode chunk binary");
    let mut expected = PacketOptions::default();
    assert!(expected.with_chunking(69, 4321).is_ok());
    assert_eq!(opts, expected);
}

#[test]
fn decode_chunk_data_text() {
    let raw = RawData::Text("0:0:1337:2560".into());
    let opts = PacketOptions::decode(raw).expect("should decode chunk text");
    let mut expected = PacketOptions::default();
    assert!(expected.with_chunking(1337, 2560).is_ok());
    assert_eq!(opts, expected);
}

#[test]
fn decode_full_options_binary() {
    // compress=1, encrypt=1, seq=48 (0x0030), total=1090 (0x0442)
    let raw = RawData::Binary(vec![1, 1, 0, 48, 4, 66]);
    let opts = PacketOptions::decode(raw).expect("should decode full options binary");
    let mut expected = PacketOptions::default()
        .with_compression()
        .with_encryption();
    expected.with_chunking(48, 1090).unwrap();
    assert_eq!(opts, expected);
}

#[test]
fn decode_full_options_text() {
    let raw = RawData::Text("1:1:97:65535".into());
    let opts = PacketOptions::decode(raw).expect("should decode full options text");
    let mut expected = PacketOptions::default()
        .with_compression()
        .with_encryption();
    expected.with_chunking(97, u16::MAX).unwrap();
    assert_eq!(opts, expected);
}

#[test]
fn decode_invalid_binary_length() {
    // too short
    let raw = RawData::Binary(vec![1, 0, 0]);
    let err = PacketOptions::decode(raw).unwrap_err();
    assert!(matches!(
        err,
        DecodingError::Packet(PacketError::InvalidPacketOptions)
    ));
}

#[test]
fn decode_invalid_text_format() {
    // wrong number of parts
    let raw = RawData::Text("1:0:0".into());
    let err = PacketOptions::decode(raw).unwrap_err();
    assert!(matches!(
        err,
        DecodingError::Packet(PacketError::InvalidPacketOptions)
    ));
}
