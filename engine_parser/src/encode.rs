use base64::{Engine as _, engine::general_purpose};

use crate::constants::{RawData, SEPARATOR_BYTE, BINARY_MASK, PLAIN_TEXT_MASK, BinaryType};
use crate::packet::Packet;

impl Packet {
    pub fn encode(&self, supports_binary: bool) -> RawData {
        match &self.data {
            // Binary data, return if supported else encode to base64
            Some(RawData::Binary(data)) => {
                if supports_binary {
                    self.data.clone().unwrap()
                } else {
                    let encoded = format!("b{}", general_purpose::URL_SAFE.encode(data));
                    RawData::Text(encoded)
                }
            }
            // Text data, format with packet type prefix
            Some(RawData::Text(text)) => {
                let encoded = format!("{}{}", self._type.as_char(), text);
                RawData::Text(encoded)
            }
            // No data, just packet type
            None => RawData::Text(self._type.as_char().to_string()),
        }
    }

    pub fn encode_binary(&self) -> BinaryType {
        match &self.data {
            Some(RawData::Binary(data)) => data.clone(),
            Some(RawData::Text(text)) => {
                let encoded_text = format!("{}{}", self._type.as_char(), text);
                encoded_text.into_bytes()
            }
            None => Vec::from([self._type.as_char() as u8]), // No data, encode type
        }
    }

    pub fn encode_payload(packets: &[Self], supports_binary: bool) -> BinaryType {
        let size = packets.len();
        let mut payload = Vec::with_capacity(packets.len());
        for (i, packet) in packets.iter().enumerate() {
            let encoded = packet.encode(supports_binary);
            match encoded {
                RawData::Text(text) => {
                    payload.push(PLAIN_TEXT_MASK);
                    payload.extend_from_slice(text.as_bytes());
                }
                RawData::Binary(binary) => {
                    payload.push(BINARY_MASK);
                    payload.extend_from_slice(&binary);
                }
            }
            if i < size - 1 {
                payload.push(SEPARATOR_BYTE);
            }
        }
        payload
    }
}