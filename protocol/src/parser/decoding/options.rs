use super::PacketOptions;
use crate::constants::RawData;
use crate::packet::error::PacketError;
use crate::parser::error::DecodingError;

impl PacketOptions {
    /// Decodes PacketOptions from RawData, choosing binary or text based on the data type.
    pub fn decode(data: RawData) -> Result<Self, DecodingError> {
        match data {
            RawData::Binary(bytes) => Self::decode_binary(&bytes),
            RawData::Text(text) => Self::decode_text(&text),
        }
    }

    /// Decodes PacketOptions from a compact byte array.
    /// Format: [compress(1), encrypted(1), sequence(2), total_chunks(2)]
    pub fn decode_binary(bytes: &[u8]) -> Result<Self, DecodingError> {
        if bytes.len() != 6 || bytes[0] > 1 || bytes[1] > 1 {
            return Err(DecodingError::DecodeError(PacketError::InvalidPacketOptions));
        }

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
    pub fn decode_text(s: &str) -> Result<Self, DecodingError> {
        let parts: Vec<_> = s.split(':').collect();
        if parts.len() != 4 { return Err(DecodingError::DecodeError(PacketError::InvalidPacketOptions)); }

        // Check compress/encrypted are "0" or "1"
        let compress = match parts[0] {
            "0" => false,
            "1" => true,
            _ => return Err(DecodingError::DecodeError(PacketError::InvalidPacketOptions)),
        };
        let encrypted = match parts[1] {
            "0" => false,
            "1" => true,
            _ => return Err(DecodingError::DecodeError(PacketError::InvalidPacketOptions)),
        };

        let sequence = parts[2].parse::<u16>()
            .map_err(|_| DecodingError::DecodeError(PacketError::InvalidPacketOptions))?;
        let total_chunks = parts[3].parse::<u16>()
            .map_err(|_| DecodingError::DecodeError(PacketError::InvalidPacketOptions))?;

        if total_chunks < 2 || sequence > total_chunks || sequence < 1 {
            return Err(DecodingError::DecodeError(PacketError::InvalidChunkingParameters));
    }

        Ok(PacketOptions {
            compress,
            sequence,
            total_chunks,
            encrypted,
        })
    }
}