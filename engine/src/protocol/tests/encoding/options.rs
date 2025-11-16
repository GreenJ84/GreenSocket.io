use crate::protocol::{
    RawData,
    PacketOptions,
};

#[test]
fn default_binary() {
    let encoded = PacketOptions::default().encode(true);
    assert_eq!(encoded, RawData::Binary(vec![0, 0, 0, 0, 0, 0]));
}

#[test]
fn default_text() {
    let encoded = PacketOptions::default().encode(false);
    assert_eq!(encoded, RawData::Text("0:0:0:0".into()));
}

#[test]
fn compress_binary() {
    let opts = PacketOptions::default().with_compression();
    let encoded = opts.encode(true);
    assert_eq!(encoded, RawData::Binary(vec![1, 0, 0, 0, 0, 0]));
}

#[test]
fn compress_text() {
    let opts = PacketOptions::default().with_compression();
    let encoded = opts.encode(false);
    assert_eq!(encoded, RawData::Text("1:0:0:0".into()));
}

#[test]
fn encrypt_binary() {
    let opts = PacketOptions::default().with_encryption();
    let encoded = opts.encode(true);
    assert_eq!(encoded, RawData::Binary(vec![0, 1, 0, 0, 0, 0]));
}

#[test]
fn encrypt_text() {
    let opts = PacketOptions::default().with_encryption();
    let encoded = opts.encode(false);
    assert_eq!(encoded, RawData::Text("0:1:0:0".into()));
}

#[test]
fn compress_and_encrypt_binary() {
    let opts = PacketOptions::default().with_compression().with_encryption();
    let encoded = opts.encode(true);
    assert_eq!(encoded, RawData::Binary(vec![1, 1, 0, 0, 0, 0]));
}

#[test]
fn compress_and_encrypt_text() {
    let opts = PacketOptions::default()
      .with_compression()
      .with_encryption();
    let encoded = opts.encode(false);
    assert_eq!(encoded, RawData::Text("1:1:0:0".into()));
}

#[test]
fn chunk_data_binary() {
    let mut opts = PacketOptions::default();
    opts.with_chunking(69, 4321).ok();
    let encoded = opts.encode(true);

    assert_eq!(encoded, RawData::Binary(vec![0, 0, 0, 69, 16, 225]));
}

#[test]
fn chunk_data_text() {
    let mut opts = PacketOptions::default();
    opts.with_chunking(1337, 2560).ok();
    let encoded = opts.encode(false);
    assert_eq!(encoded, RawData::Text("0:0:1337:2560".into()));
}

#[test]
fn full_options_binary() {
    let mut opts = PacketOptions::default()
      .with_compression()
      .with_encryption();
    opts.with_chunking(48, 1090).ok();
    let encoded = opts.encode(true);
    assert_eq!(encoded, RawData::Binary(vec![1, 1, 0, 48, 4, 66]));
}

#[test]
fn full_options_text() {
    let mut opts = PacketOptions::default()
      .with_compression()
      .with_encryption();
    opts.with_chunking(97, u16::MAX).ok();
    let encoded = opts.encode(false);
    assert_eq!(encoded, RawData::Text("1:1:97:65535".into()));
}