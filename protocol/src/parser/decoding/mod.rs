pub(crate) mod options;

use base64::{Engine as _, engine::general_purpose};
use crate::{
    Packet,
    PacketType,
    PacketError,
    PacketOptions,
    RawData,
    BinaryType,
    constants::BINARY_MASK,
    constants::PLAIN_TEXT_MASK,
    DecodingError
};

impl Packet {
    pub fn decode(encoded_packet: RawData) -> Result<Self, DecodingError> {
        match encoded_packet {
            RawData::Binary(data) => Self::decode_binary(data),
            RawData::Text(s) => Self::decode_text(s),
        }
    }

    fn decode_binary(encoded: BinaryType) -> Result<Self, DecodingError> {
        let encoded_len = encoded.len();
        if encoded_len < 3 { return Err(DecodingError::MissingField); }

        let _type = PacketType::try_from(encoded[0])
            .map_err(|e| DecodingError::Packet(e))?;
        let mut packet = Packet::new(_type);

        if encoded[1] > 1 || encoded[2] > 1 {
            return Err(DecodingError::InvalidFormat);
        }
        let has_options = encoded[1] == 1;
        let has_data = encoded[2] == 1;
        if !has_options && !has_data { return Ok(packet); }

        if has_options {
            if encoded_len < 9 { return Err(DecodingError::Packet(PacketError::InvalidPacketOptions)); }
            let opts = PacketOptions::decode(
                RawData::Binary(encoded[3..10].to_vec())
            )?;
            packet.with_options(opts);
        }

        if has_data {
            let data_type = encoded[10];
            let data: RawData = match data_type {
                BINARY_MASK => Ok(RawData::Binary(encoded[11..].to_vec())),
                PLAIN_TEXT_MASK => Ok(RawData::Text(String::from_utf8_lossy(&encoded[11..]).into())),
                _ => Err(DecodingError::InvalidFormat),
            }?;
            packet.with_data(data)
                .map_err(|e| DecodingError::Packet(e))?;
        }
        Ok(packet)
    }

    fn decode_text(encoded: String) -> Result<Self, DecodingError> {
        if encoded.len() < 3 { return Err(DecodingError::MissingField); }

        let mut chars = encoded.chars();
        let _type = match PacketType::try_from(chars.next().unwrap_or('X')) {
            Ok(t) => t,
            Err(_) => return Err(DecodingError::Packet(PacketError::InvalidPacketType)),
        };
        let mut packet = Packet::new(_type);

        let has_options = match chars.next().unwrap(){
            '0' => false,
            '1' => true,
            _ => return Err(DecodingError::InvalidFormat),
        };
        let has_data = match chars.next().unwrap(){
            '0' => false,
            '1' => true,
            _ => return Err(DecodingError::InvalidFormat),
        };
        if !has_options && !has_data { return Ok(packet); }

        if has_options {
            if encoded.len() < 10 { return Err(DecodingError::Packet(PacketError::InvalidPacketOptions)); }

            let raw_options = chars.by_ref().take(7).collect();
            let options = PacketOptions::decode(RawData::Text(raw_options))?;
            packet.with_options(options);
        }

        if has_data {
            let data = chars.collect::<String>();
            match &data[..1] {
                "b" => {
                    let b64_data = &data[1..];
                    match general_purpose::STANDARD.decode(b64_data) {
                        Ok(bytes) => packet.with_data(RawData::Binary(bytes))
                            .map_err(|e| DecodingError::Packet(e))?,
                        Err(e) => return Err(DecodingError::Base64(e)),
                    };
                },
                "t" => {
                    packet.with_data(RawData::Text(data[1..].to_owned())).map_err(|e| DecodingError::Packet(e))?;
                },
                _ => return Err(DecodingError::InvalidFormat)
            };
        }

        Ok(packet)
    }

    /// Decodes a payload of packets.
    pub fn decode_payload(encoded: RawData) -> Result<Vec<Self>, DecodingError> {
        let mut payload = Vec::<Self>::new();

        match encoded {
            RawData::Binary(bin) => {
                if bin.is_empty() { return Ok(payload); }

                let mut bytes = bin.into_iter().peekable();
                loop {
                    if bytes.by_ref().peek().is_none() { break; }
                    let len_prefix: Vec<u8> = bytes.by_ref().take(4).collect();

                    if len_prefix.len() < 4 { return Err(DecodingError::PayloadDataMismatch); }
                    let len = u32::from_be_bytes(
                        [len_prefix[0], len_prefix[1], len_prefix[2], len_prefix[3]]
                    ) as usize;

                    let chunk: Vec<u8> = bytes.by_ref().take(len).collect();
                    if chunk.len() < len { return Err(DecodingError::PayloadDataMismatch); }

                    let decoded = Self::decode(RawData::Binary(chunk))?;
                    payload.push(decoded);
                }
                Ok(payload)
            },
            RawData::Text(txt) => {
                if txt.is_empty() { return Ok(payload); }
                let mut chars = txt.chars().peekable();

                loop {
                    if chars.by_ref().peek().is_none() { break; }

                    let len_str: String = chars.by_ref().take(8).collect();
                    let len = len_str.parse::<usize>()
                        .map_err(|_| DecodingError::PayloadDataMismatch)?;

                    let chunk: String = chars.by_ref().take(len).collect();
                    if chunk.len() < len { return Err(DecodingError::PayloadDataMismatch); }

                    let decoded = Self::decode(RawData::Text(chunk.into()))?;
                    payload.push(decoded);
                }
                Ok(payload)
            }
        }
    }
}

