use base64::{Engine as _, engine::general_purpose};

use crate::constants::*;
use super::Packet;

impl Packet {
    /// Binary Data encoding always has:
        /// - First bit set for a TEXT vs Binary flag
        /// - Second bit set for Packet type
        /// - Rest for data
    /// Text Data encoding always has:
        /// - First char -- as 'b' for Binary data -- as packet_type for Type
        /// - Second char as packet_type for binary data
        /// - Rest for data
    pub fn encode(&self, supports_binary: bool) -> RawData {
        match &self.data {
            // Binary data, return if supported else encode to base64
            Some(RawData::Binary(data)) => {
                if supports_binary {
                    let mut bin = Vec::from(
                        [BINARY_MASK, self._type.as_char() as u8]
                    );
                    bin.extend_from_slice(&data);
                    RawData::Binary(bin)
                } else {
                    let mut encoded = format!("b{}", self._type.as_char());
                    encoded.push_str(&general_purpose::URL_SAFE.encode(data));
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
        let mut res = Vec::<u8>::new();
        match &self.data {
            Some(RawData::Binary(_)) => {
                match self.encode(true) {
                    RawData::Binary(encoded_data) => res.extend_from_slice(&encoded_data),
                    _ => {
                        // This should never happen, but handle gracefully
                        eprintln!("Warning: Binary encoding returned non-binary data");
                        res.extend_from_slice(&[PLAIN_TEXT_MASK, self._type.as_char() as u8]);
                    }
                }
            },
            Some(RawData::Text(_)) => {
                res.push(PLAIN_TEXT_MASK);
                match self.encode(false) {
                    RawData::Text(encoded_text) => res.extend_from_slice(&encoded_text.into_bytes()),
                    _ => {
                        // This should never happen, but handle gracefully
                        eprintln!("Warning: Text encoding returned non-text data");
                        res.extend_from_slice(&[self._type.as_char() as u8]);
                    }
                }
            }
            None => { res.extend_from_slice(&[PLAIN_TEXT_MASK, self._type.as_char() as u8]); }
        }
        res
    }


    pub fn encode_payload(packets: &[Self], supports_binary: bool) -> RawData {
        let size = packets.len();
        return if supports_binary {
            let mut payload = Vec::<u8>::with_capacity(size * 2);
            for (i, packet) in packets.iter().enumerate() {
                let encoded = packet.encode_binary();
                payload.extend(&encoded);
                if i < size - 1 {
                    payload.push(SEPARATOR_BYTE);
                }
            }
            RawData::Binary(payload)
        } else {
            let mut payload = String::with_capacity(size * 2);
            for (i, packet) in packets.iter().enumerate() {
                if let RawData::Text(encoded) = packet.encode(supports_binary) {
                    payload.push_str(&encoded);
                }
                if i < size - 1 {
                    payload.push(char::from(SEPARATOR_BYTE));
                }
            }
            RawData::Text(payload)
        };
    }
}