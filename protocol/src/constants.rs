use serde::{Deserialize, Serialize};

pub(crate) const _PROTOCOL: u8 = 4;

pub(crate) const SEPARATOR_BYTE: u8 = 30;
pub(crate) const PLAIN_TEXT_MASK: u8 = 0x00;
pub(crate) const BINARY_MASK: u8 = 0x80;

pub type BinaryType = Vec<u8>;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum RawData {
    Text(String),
    Binary(BinaryType)
}
impl RawData {
    pub fn len(&self) -> usize {
        match self {
            RawData::Binary(b) => b.len(),
            RawData::Text(s) => s.len(),
        }
    }
    pub fn is_binary(&self) -> bool{
        match self {
            RawData::Binary(_) => true,
            RawData::Text(_) => false,
        }
    }
    pub fn is_text(&self) -> bool{
        match self {
            RawData::Binary(_) => false,
            RawData::Text(_) => true,
        }
    }
}
