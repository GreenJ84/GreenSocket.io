pub(crate) mod options;

use base64::{Engine as _, engine::general_purpose};

use crate::{
    Packet,
    RawData,
    BinaryType,
    constants::BINARY_MASK,
    constants::PLAIN_TEXT_MASK,
};

impl Packet {
    /// Encodes the packet as either binary or text, depending on supports_binary.
    pub fn encode(self, supports_binary: bool) -> RawData {
        match supports_binary {
            true => RawData::Binary(self.encode_binary()),
            false => RawData::Text(self.encode_text()),
        }
    }

    /// Encodes the packet as binary.
    /// [PacketType (1 byte), PacketOptions (6 bytes), Data prefix (1 byte), Data (variable)]
    fn encode_binary(self) -> BinaryType {
        let mut bin = Vec::<u8>::new();
        bin.push(self._type().to_owned().into());
        bin.push(
            if self.options().is_some() { 1 } else { 0 }
        );
        bin.push(
            if self.data().is_some() { 1 } else { 0 }
        );

        if let Some(opts) = self.options() {
            match opts.to_owned().encode(true) {
                RawData::Binary(bytes) => bin.extend_from_slice(&bytes),
                _ => {
                    debug_assert!(false, "PacketOptions.encode(true) did not return RawData::Binary");
                    eprintln!("Warning: PacketOptions did not encode!");
                }
            }
        }

        match self.data() {
            Some(RawData::Binary(data)) => {
                bin.push(BINARY_MASK);
                bin.extend_from_slice(data);
            }
            Some(RawData::Text(text)) => {
                bin.push(PLAIN_TEXT_MASK);
                bin.extend_from_slice(text.as_bytes());
            }
            None => {}
        }
        bin
    }

    /// Encodes the packet as text.
    /// Format: "<packet_type><options><data_prefix><data>"
    fn encode_text(self) -> String {
        let mut encoded = String::new();
        encoded.push(self._type().to_owned().into());
        encoded.push(
            if self.options().is_some() { '1' } else { '0' }
        );
        encoded.push(
            if self.data().is_some() { '1' } else { '0' }
        );

        if let Some(opts) = self.options() {
            match opts.to_owned().encode(false) {
                RawData::Text(text) => encoded.push_str(&text),
                _ => {
                    debug_assert!(false, "PacketOptions.encode(false) did not return RawData::Text");
                    eprintln!("Warning: PacketOptions did not encode!");
                }
            }
        }

        if let Some(data) = self.data() {
            encoded.push('-');
            match data {
                RawData::Binary(data) => {
                    encoded.push('b');
                    encoded.push_str(&general_purpose::URL_SAFE.encode(data));
                },
                RawData::Text(text) => {
                    encoded.push('t');
                    encoded.push_str(text);
                }
            }
        }
        encoded
    }

    /// Encodes a payload of packets.
    pub fn encode_payload(packets: Vec<Self>, supports_binary: bool) -> RawData {
        match supports_binary {
            true => {
                let mut payload = Vec::<u8>::new();
                for packet in packets {
                    let encoded = packet.encode_binary();
                    let len = encoded.len() as u32;
                    payload.extend_from_slice(&len.to_be_bytes());
                    payload.extend(encoded);
                }
                RawData::Binary(payload)
            },
            _ => {
                let mut payload = String::new();
                for packet in packets {
                    let encoded = packet.encode_text();
                    let len = encoded.len();
                    payload.push_str(&format!("{:08}{}", len, encoded));
                }
                RawData::Text(payload)
            }
        }
    }
}