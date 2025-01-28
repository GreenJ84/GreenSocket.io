pub(crate) const _PROTOCOL: u8 = 4;

pub(crate) const SEPARATOR_BYTE: u8 = 30;
pub(crate) const PLAIN_TEXT_MASK: u8 = 0x00;
pub(crate) const BINARY_MASK: u8 = 0x80;

pub type BinaryType = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawData {
    Text(String),
    Binary(BinaryType)
}