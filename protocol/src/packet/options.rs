use super::error::PacketError;

/// Options for packet transmission. "Packet Headers"
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct PacketOptions {
    /// Whether the packet should be compressed.
    pub compress: bool,
    /// Whether the packet is encrypted.
    pub encrypted: bool,
    /// The sequence number of the packet (for chunked transfer).
    pub sequence: Option<u16>,
    /// The total number of chunks in the packet (for chunked transfer).
    pub total_chunks: Option<u16>,
}


impl PacketOptions {
    /// Creates a new PacketOptions with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables compression for the packet.
    pub fn with_compression(mut self) -> Self {
        self.compress = true;
        self
    }

    /// Enables encryption for the packet.
    pub fn with_encryption(mut self) -> Self {
        self.encrypted = true;
        self
    }

    /// Sets chunking information for the packet.
    pub fn with_chunking(mut self, sequence: u16, total_chunks: u16) -> Result<Self, PacketError> {
        if total_chunks < 2 || sequence > total_chunks || sequence < 1 {
            eprintln!("Invalid chunking parameters");
            return Err(PacketError::InvalidChunkingParameters);
        }
        self.sequence = Some(sequence);
        self.total_chunks = Some(total_chunks);
        Ok(self)
    }
}
