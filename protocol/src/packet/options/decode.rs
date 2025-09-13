use super::PacketOptions;
use crate::error::ProtocolError;
use crate::constants::RawData;

impl PacketOptions {
    /// Decodes PacketOptions from RawData, choosing binary or text based on the data type.
    pub fn decode(data: RawData) -> Result<Self, ProtocolError> {
        match data {
            RawData::Binary(bytes) => Self::decode_binary(&bytes),
            RawData::Text(text) => Self::decode_text(&text),
        }
    }

    /// Decodes PacketOptions from a compact byte array.
    /// Format: [compress(1), encrypted(1), sequence(2), total_chunks(2)]
    pub fn decode_binary(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() != 10 { return Err(ProtocolError::DecodeError); }
        let compress = bytes[0] == 1;
        let encrypted = bytes[1] == 1;

        let sequence = u16::from_be_bytes([bytes[2], bytes[3]]);
        let total_chunks = u16::from_be_bytes([bytes[4], bytes[5]]);

        Ok(PacketOptions {
            compress,
            encrypted,
            sequence: if sequence > 0 { Some(sequence as u16) } else { None },
            total_chunks: if total_chunks > 0 { Some(total_chunks as u16) } else { None },
        })
    }

    /// Decodes PacketOptions from a compact string.
    /// Format: "compress:encrypted:sequence:total_chunks"
    pub fn decode_text(s: &str) -> Result<Self, ProtocolError> {
        let parts: Vec<_> = s.split(':').collect();
        if parts.len() != 4 { return Err(ProtocolError::DecodeError); }

        let compress = parts[0] == "1";
        let encrypted = parts[1] == "1";

        let sequence = parts[2].parse::<u16>().ok();
        let total_chunks = parts[3].parse::<u16>().ok();

        Ok(PacketOptions {
            compress,
            sequence,
            total_chunks,
            encrypted,
        })
    }
}