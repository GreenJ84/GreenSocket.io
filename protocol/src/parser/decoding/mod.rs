use base64::{Engine as _, engine::general_purpose};

use crate::constants::*;
use crate::packet::{Packet, PacketType};

impl Packet {
    pub fn decode(encoded_packet: RawData) -> Self {
        let packet_len = encoded_packet.len();
        if packet_len == 0 {
            return Self::new(PacketType::Noop, None, None);
        }
        match encoded_packet {
            RawData::Binary(data) => Self::decode_binary(data),
            RawData::Text(s) => Self::decode_text(s),
        }
    }

    fn decode_binary(encoded: BinaryType) -> Result<Self, ProtocolError> {
        let encoded_len = encoded.len();
        if encoded_len < 3 { return Err(ProtocolError::DecodeError); }

        let _type = PacketType::from_int(encoded[0])?;
        let mut packet = Packet::new(_type);

        let has_options = encoded[1] == 1;
        let has_data = encoded[2] == 1;
        if !has_options && !has_data { return Ok(packet); }

        if has_options {
            if encoded_len < 10 { return Err(ProtocolError::InvalidPacketOptions); }
            let opts = PacketOptionvhjns::decode(
                RawData::Binary(encoded[3..10].to_vec())
            )?;
            packet.with_options(opts);
        }

        if has_data {
            let data_type = encoded[10];
            let data: RawData = match data_type {
                BINARY_MASK => RawData::Binary(encoded[11..].to_vec()),
                _ => RawData::Text(String::from_utf8_lossy(&encoded[11..]).into()),
            };
            packet.with_data(data)?;
        }
        Ok(packet)
    }

    fn decode_text(encoded: String) -> Result<Self, ProtocolError> {
        let mut chars = encoded.chars();
        let packet_type_char = chars.next().unwrap_or('X');
        let _type = match PacketType::from_char(packet_type_char) {
            Some(t) => t,
            None => return Err(ProtocolError::InvalidPacketType),
        };
        let mut packet = Packet::new(_type);

        let rest_of_string: String = chars.collect();
        if rest_of_string.is_empty() { return packet; }

        let mut parts = rest_of_string.splitn(2, PLAIN_TEXT_MASK);
        let options_part = parts.next().unwrap_or("");
        let data_part = parts.next();

        if !options_part.is_empty() {
            if let Ok(opts) = PacketOptions::decode(RawData::Text(options_part.to_string())) {
                packet.with_options(opts);
            } else {
                return Packet::error("Invalid packet options encoding");
            }
        }

        if let Some(data_str) = data_part {
            let data = RawData::Text(data_str.to_string());
            if let Err(e) = packet.with_data(data) {
                return Packet::error(&format!("Data too large: {:?}", e));
            }
        }
        packet
    }

    /// Decodes a payload of packets.
    pub fn decode_payload(encoded: RawData) -> Vec<Self> {
        match encoded {
            RawData::Text(txt) => {
                if txt.is_empty() { return Vec::new(); }

                let mut chunks: Vec<String> = txt
                    .split(char::from(SEPARATOR_BYTE))
                    .map(|s| s.to_owned())
                    .collect();
                // println!("{:?}", chunks);
                let mut payload = Vec::<Self>::with_capacity(chunks.len());

                for chunk in chunks.drain(..) {
                    let decoded = Self::decode(RawData::Text(chunk.into()));
                    payload.push(decoded);
                }
                payload
            },
            RawData::Binary(bin) => {
                if bin.is_empty() { return Vec::new(); }

                let mut chunks: Vec<BinaryType> = bin
                    .split(|n| n == &SEPARATOR_BYTE)
                    .map(|s| s.to_vec())
                    .collect();
                // println!("{:?}", chunks);
                let mut payload = Vec::<Self>::with_capacity(chunks.len());

                for chunk in chunks.drain(..) {
                    let decoded = Self::decode(RawData::Binary(chunk));
                    payload.push(decoded);
                }
                payload
            }
        }
    }
}

