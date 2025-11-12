use crate::{
    PacketError,
    PacketOptions,
    RawData,
    BinaryType,
    DecodingError
};

impl PacketOptions {
    /// Decodes PacketOptions from RawData, choosing binary or text based on the data type.
    pub fn decode(data: RawData) -> Result<Self, DecodingError> {
        match data {
            RawData::Binary(bytes) => Self::decode_binary(bytes),
            RawData::Text(text) => Self::decode_text(text),
        }
    }

    /// Decodes PacketOptions from a compact byte array.
    /// Format: [compress(1), encrypted(1), sequence(2), total_chunks(2)]
    pub fn decode_binary(bytes: BinaryType) -> Result<Self, DecodingError> {
        if bytes.len() != 6 || bytes[0] > 1 || bytes[1] > 1 {
            return Err(DecodingError::Packet(PacketError::InvalidPacketOptions));
        }
        let mut options = PacketOptions::default();

        if bytes[0] == 1 {
            options = options.with_compression();
        }
        if bytes[1] == 1 {
            options = options.with_encryption();
        }

        let sequence = u16::from_be_bytes([bytes[2], bytes[3]]);
        let total_chunks = u16::from_be_bytes([bytes[4], bytes[5]]);
        if total_chunks == 0 && sequence == 0 {
            return Ok(options);
        }
        options.with_chunking(sequence, total_chunks)
            .map_err(|e| DecodingError::Packet(e))?;

        Ok(options)
    }

    /// Decodes PacketOptions from a compact string.
    /// Format: "compress:encrypted:sequence:total_chunks"
    pub fn decode_text(s: String) -> Result<Self, DecodingError> {
        let parts: Vec<_> = s.split(':').collect();
        if parts.len() != 4 { return Err(DecodingError::Packet(PacketError::InvalidPacketOptions)); }
        let mut options = PacketOptions::default();

        let compress = match parts[0] {
            "0" => false,
            "1" => true,
            _ => return Err(DecodingError::Packet(PacketError::InvalidPacketOptions)),
        };
        if compress {
            options = options.with_compression();
        }

        let encrypt = match parts[1] {
            "0" => false,
            "1" => true,
            _ => return Err(DecodingError::Packet(PacketError::InvalidPacketOptions)),
        };
        if encrypt {
            options = options.with_encryption();
        }

        let sequence = parts[2].parse::<u16>()
            .map_err(|_| DecodingError::Packet(PacketError::InvalidPacketOptions))?;
        let total_chunks = parts[3].parse::<u16>()
            .map_err(|_| DecodingError::Packet(PacketError::InvalidPacketOptions))?;

        if total_chunks == 0 && sequence == 0 {
            return Ok(options);
        }
        options.with_chunking(sequence, total_chunks)
            .map_err(|e| DecodingError::Packet(e))?;

        Ok(options)
    }
}