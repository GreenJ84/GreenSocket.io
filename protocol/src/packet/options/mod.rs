pub(crate) mod encode;
pub(crate) mod decode;

/// Options for packet transmission. "Packet Headers"
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct PacketOptions {
    /// Whether the packet should be compressed.
    pub compress: bool,
    /// Whether the packet is encrypted.
    pub encrypted: bool,
    /// The sequence number of the packet (for chunked transfer).
    pub sequence: Option<usize>,
    /// The total number of chunks in the packet (for chunked transfer).
    pub total_chunks: Option<usize>,
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
    pub fn with_chunking(mut self, sequence: usize, total_chunks: usize) -> Self {
        if total_chunks < 2 || sequence < 1 { return self }
        else if sequence > total_chunks {
            eprintln!("Invalid chunking parameters: sequence must be less than total_chunks and total_chunks must be greater than 0");
            return self;
        }
        self.sequence = Some(sequence);
        self.total_chunks = Some(total_chunks);
        self
    }
}
