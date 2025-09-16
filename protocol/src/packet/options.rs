use crate::PacketError;

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
    /// Creates a new `PacketOptions` instance with specified parameters.
    pub fn new(compress: bool, encrypt: bool, sequence: Option<u16>, total_chunks: Option<u16>) -> Result<Self, PacketError> {
        let mut options = Self::default();

        options.compress = compress;
        options.encrypt = encrypt;
        if let (Some(seq), Some(total)) = (sequence, total_chunks) {
            options.with_chunking(seq, total)?;
        } else if sequence.is_some() || total_chunks.is_some() {
            return Err(PacketError::InvalidChunkingParameters);
        }
        Ok(options)
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
    pub fn with_chunking(&mut self, sequence: u16, total_chunks: u16) -> Result<(), PacketError> {
        if sequence > total_chunks || sequence == 0 || total_chunks == 0 {
            eprintln!("Invalid chunking parameters");
            return Err(PacketError::InvalidChunkingParameters);
        }

        self.sequence = Some(sequence);
        self.total_chunks = Some(total_chunks);
        Ok(())
    }
}
