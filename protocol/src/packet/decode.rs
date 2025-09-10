use base64::{Engine as _, engine::general_purpose};

use crate::constants::*;
use super::{Packet, PacketType};

impl Packet {
    pub fn decode(encoded_packet: RawData) -> Self {
        let packet_len = encoded_packet.len();
        if packet_len == 0 {
            return Self::new(PacketType::Noop, None, None);
        }
        match encoded_packet {
            RawData::Binary(data) => {
                match data[0] {
                    BINARY_MASK => {
                        match PacketType::from_char(char::from(data[1])) {
                            Some(packet_type) => Self {
                                _type: packet_type,
                                options: None,
                                data: if data.len() < 3 { None } else {
                                    Some(RawData::Binary(Vec::from(&data[2..])))
                                },
                            },
                            _ => {
                                return Packet::error("Invalid binary packet type flag");
                            }
                        },
                    },
                    PLAIN_TEXT_MASK => {
                        match String::from_utf8(Vec::from(&data[1..])) {
                            Ok(text_data) => Packet::decode(RawData::Text(text_data)),
                            Err(e) => Packet::error(&format!("Binary parsing error: {:?}", e))
                        }
                    },
                    _ => { Self::error("Invalid binary packet - missing association flag") }
                }
            },
            RawData::Text(s) => {
                let mut chars = s.chars();
                match chars.nth(0) {
                    Some('b') => {
                        if let Some(packet_type) = PacketType::from_char(chars.nth(0).unwrap_or('X')) {
                            if packet_len == 2 { return Self::new(packet_type, None, None); }

                            if let Ok(data) = general_purpose::URL_SAFE.decode(&s[2..]) {
                                return Self {
                                    _type: packet_type,
                                    options: None,
                                    data: Some(RawData::Binary(data)),
                                };
                            }
                            return Self::error("Invalid base64 encoding of binary data");
                        }
                        Packet::error("Invalid packet type flag on encoded data")
                    },
                    Some(c) => {
                        if let Some(packet_type) = PacketType::from_char(c) {
                            return Self {
                                _type: packet_type,
                                options: None,
                                data: if s.len() < 2 { None } else {
                                    Some(RawData::Text(s[1..].to_string()))
                                }
                            }
                        }
                        Self::error("Invalid packet type for text encoded data")
                    },
                    None => Self::error("Invalid format for text encoded data")
                }
            }
        }
    }

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

