use crate::{PacketOptions, PacketError};

#[test]
fn default_has_options_disabled() {
    let opts = PacketOptions::default();
    assert!(!opts.compress());
    assert!(!opts.encrypt());
    assert_eq!(opts.sequence(), None);
    assert_eq!(opts.total_chunks(), None);
}

#[test]
fn enable_compression() {
    let opts = PacketOptions::default()
      .with_compression();
    assert!(opts.compress());
    assert!(!opts.encrypt());
    assert_eq!(opts.sequence(), None);
    assert_eq!(opts.total_chunks(), None);
}

#[test]
fn enable_encryption() {
    let opts = PacketOptions::default()
      .with_encryption();
    assert!(opts.encrypt());
    assert!(!opts.compress());
    assert_eq!(opts.sequence(), None);
    assert_eq!(opts.total_chunks(), None);
}

#[test]
fn enable_both_compression_and_encryption() {
    let opts = PacketOptions::default()
      .with_compression()
      .with_encryption();
    assert!(opts.compress());
    assert!(opts.encrypt());
    assert_eq!(opts.sequence(), None);
    assert_eq!(opts.total_chunks(), None);
}

#[test]
fn set_and_get_valid_chunking() {
    let opts = PacketOptions::default()
      .with_chunking(2, 4).unwrap();
    assert!(!opts.compress());
    assert!(!opts.encrypt());
    assert_eq!(opts.sequence(), Some(2));
    assert_eq!(opts.total_chunks(), Some(4));
}

#[test]
fn invalid_chunking_seq_gt_total() {
    let result = PacketOptions::default().with_chunking(5, 4);
    assert!(matches!(result, Err(PacketError::InvalidChunkingParameters)));
}

#[test]
fn valid_new_packet_options_1() {
    let opts = PacketOptions::new(
      true,
      true,
      Some(1),
      Some(2)
    ).unwrap();
    assert!(opts.compress());
    assert!(opts.encrypt());
    assert_eq!(opts.sequence(), Some(1));
    assert_eq!(opts.total_chunks(), Some(2));
}

#[test]
fn valid_new_packet_options_2() {
    let opts = PacketOptions::new(
      false,
      true,
      None,
      None
    ).unwrap();
    assert!(!opts.compress());
    assert!(opts.encrypt());
    assert_eq!(opts.sequence(), None);
    assert_eq!(opts.total_chunks(), None);
}

#[test]
fn invalid_new_options_0_chunks() {
    let opts = PacketOptions::new(
      true,
      true,
      Some(0),
      Some(0)
    );
    assert!(matches!(opts, Err(PacketError::InvalidChunkingParameters)));
}

#[test]
fn invalid_new_options_seq_gt_total() {
    let opts = PacketOptions::new(
      true,
      true,
      Some(21),
      Some(12)
    );
    assert!(matches!(opts, Err(PacketError::InvalidChunkingParameters)));
}

#[test]
fn invalid_new_options_full_seq_empty_chunks() {
    let opts = PacketOptions::new(
      true,
      false,
      Some(12),
      None
    );
    assert!(matches!(opts, Err(PacketError::InvalidChunkingParameters)));
}

#[test]
fn options_are_copy_and_clone() {
    let opts = PacketOptions::default().with_compression();

    let opts2 = opts;
    let opts3 = opts2.clone();
    assert_eq!(opts, opts2);
    assert_eq!(opts2, opts3);
}