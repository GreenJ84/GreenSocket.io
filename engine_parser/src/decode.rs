use base64::{Engine as _, engine::general_purpose};

use crate::constants::{RawData, SEPARATOR_BYTE, BINARY_MASK, PLAIN_TEXT_MASK};
use crate::packet::{Packet, PacketType};

impl Packet {
    pub fn decode(encoded_packet: RawData) -> Self {
        match encoded_packet {
            RawData::Binary(data) => Self {
                _type: PacketType::Message,
                options: None,
                data: Some(RawData::Binary(data)),
            },
            RawData::Text(s) => {
                let mut chars = s.chars();
                if let Some(first_char) = chars.next() {
                    if first_char == 'b' {
                        let data = general_purpose::URL_SAFE.decode(&s[1..]);
                        return if let Ok(data) = data {
                            Self {
                                _type: PacketType::Message,
                                options: None,
                                data: Some(RawData::Binary(data)),
                            }
                        } else {
                            Self::error("Invalid base64 decoding")
                        }
                    }
                    if let Some(packet_type) = PacketType::from_char(first_char) {
                        return Self {
                            _type: packet_type,
                            options: None,
                            data: if s.len() <= 1 { None } else {
                                Some(RawData::Text(s[1..].to_string()))
                            },
                        };
                    }
                }
                Self::error("Invalid raw text data for packet")
            }
        }
    }

    pub fn decode_payload(encoded: &[u8]) -> Vec<Self> {
        if encoded.is_empty() { return Vec::new(); }

        let chunks: Vec<&[u8]> = encoded.split(|&b| b == SEPARATOR_BYTE).collect();
        let mut payload = Vec::<Self>::with_capacity(chunks.len());

        for chunk in chunks.iter() {
            let data_type = chunk[0]; // First byte determines type
            let data = &chunk[1..]; // Rest is the actual data

            let decoded = match data_type {
                PLAIN_TEXT_MASK => {
                    let text = String::from_utf8(data.to_vec());
                    if text.is_err() { break; }
                    Self::decode(RawData::Text(text.unwrap()))
                },
                BINARY_MASK => {
                    Self::decode(RawData::Binary(data.to_vec()))
                },
                _ => {
                    break;
                }
            };
            payload.push(decoded);
        }
        payload
    }
}

