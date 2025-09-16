use crate::{
    PacketOptions,
    RawData
};

impl PacketOptions {
    /// Encodes PacketOptions into RawData, choosing binary or text based on supports_binary.
    pub fn encode(self, supports_binary: bool) -> RawData {
        match supports_binary {
            true => RawData::Binary(self.encode_binary()),
            false => RawData::Text(self.encode_text()),
        }
    }

    /// Encodes PacketOptions as a compact byte array.
    fn encode_binary(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(self.compress() as u8);
        buffer.push(self.encrypt() as u8);

        // Encode sequence and total_chunks as u16 (2 bytes each)
        buffer.extend_from_slice(
            &(self.sequence().unwrap_or(0)).to_be_bytes()
        );
        buffer.extend_from_slice(
            &(self.total_chunks().unwrap_or(0)).to_be_bytes()
        );

        buffer
    }

    /// Encodes PacketOptions as a compact string (e.g., "1:0:10:20").
    fn encode_text(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.compress(),
            self.encrypt(),
            self.sequence().unwrap_or(0),
            self.total_chunks().unwrap_or(0),
        )
    }
}