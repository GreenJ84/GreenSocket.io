use super::error::PacketError;

/// Options for packet transmission. "Packet Headers"
#[derive(PartialEq, Eq, Debug, Default, Copy, Clone)]
pub struct PacketOptions {
    /// Whether the packet should be compressed.
    compress: bool,
    /// Whether the packet should be encrypted.
    encrypt: bool,
    /// The sequence number of the packet (for chunked transfer).
    sequence: Option<u16>,
    /// The total number of chunks in the packet (for chunked transfer).
    total_chunks: Option<u16>,
}

impl PacketOptions {
    /// Creates a new PacketOptions with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether compression is enabled.
    pub fn compress(&self) -> bool {
        self.compress
    }

    /// Enables compression for the packet.
    pub fn with_compression(mut self) -> Self {
        self.compress = true;
        self
    }

    /// Returns whether encryption is enabled.
    pub fn encrypt(&self) -> bool {
        self.encrypt
    }

    /// Enables encryption for the packet.
    pub fn with_encryption(mut self) -> Self {
        self.encrypt = true;
        self
    }

    /// Returns the sequence number if chunking is enabled.
    pub fn sequence(&self) -> Option<u16> {
        self.sequence
    }

    /// Returns the total number of chunks if chunking is enabled.
    pub fn total_chunks(&self) -> Option<u16> {
        self.total_chunks
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
