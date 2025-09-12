use base64::{Engine as _, engine::general_purpose};

use crate::constants::*;
use super::Packet;

impl Packet {
    /// Encodes the packet as either binary or text, depending on supports_binary.
    pub fn encode(&self, supports_binary: bool) -> RawData {
        match (&self.data, supports_binary) {
            (Some(_), true) => RawData::Binary(self.encode_binary()),
            (Some(_), false) => RawData::Text(self.encode_text()),
            (None, true) => RawData::Binary(vec![PLAIN_TEXT_MASK, self._type.as_char() as u8]),
            (None, false) => RawData::Text(self._type.as_char().to_string()),
        }
    }

    /// Encodes the packet as binary.
    fn encode_binary(&self) -> BinaryType {
        let mut bin = Vec::<u8>::new();
        let data_mask = match &self.data {
            Some(RawData::Binary(_)) => BINARY_MASK,
            _ => PLAIN_TEXT_MASK,
        };
        bin.push(data_mask);
        bin.push(self._type.as_char() as u8);

        if let Some(opts) = &self.options {
            match opts.encode(true) {
                RawData::Binary(bytes) => bin.extend_from_slice(&bytes),
                _ => {
                    debug_assert!(false, "PacketOptions.encode(true) did not return RawData::Binary");
                    eprintln!("Warning: PacketOptions did not encode!");
                }
            }
        }

        match &self.data {
            Some(RawData::Binary(data)) => bin.extend_from_slice(data),
            Some(RawData::Text(text)) => bin.extend_from_slice(text.as_bytes()),
            None => {}
        }
        bin
    }

    /// Encodes the packet as text.
    fn encode_text(&self) -> String {
        let mut encoded = String::new();
        let data_mask = match &self.data {
            Some(RawData::Binary(_)) => 'b',
            _ => 't',
        };
        encoded.push(data_mask);
        encoded.push(self._type.as_char());

        if let Some(opts) = &self.options {
            match opts.encode(false) {
                RawData::Text(text) => encoded.push_str(&text),
                _ => {
                    debug_assert!(false, "PacketOptions.encode(false) did not return RawData::Text");
                    eprintln!("Warning: PacketOptions did not encode!");
                }
            }
        }

        match &self.data {
            Some(RawData::Binary(data)) => {
                let b64_data = general_purpose::URL_SAFE.encode(data);
                encoded.push_str(&b64_data);
            }
            Some(RawData::Text(text)) => {
                encoded.push_str(text);
            }
            None => {}
        }
        encoded
    }

    /// Encodes a payload of packets.
    pub fn encode_payload(packets: &[Self], supports_binary: bool) -> RawData {
        if supports_binary {
            let mut payload = Vec::<u8>::new();
            for packet in packets {
                let encoded = packet.encode_binary();
                let len = encoded.len() as u32;
                payload.extend_from_slice(&len.to_be_bytes());
                payload.extend(encoded);
            }
            RawData::Binary(payload)
        } else {
            let mut payload = String::new();
            for packet in packets {
                let encoded = packet.encode_text();
                let len = encoded.len();
                payload.push_str(&format!("{:04}{}", len, encoded));
            }
            RawData::Text(payload)
        }
    }
}