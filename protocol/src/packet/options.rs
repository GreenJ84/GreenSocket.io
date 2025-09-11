use bitflags::bitflags;

/// Options for packet transmission, allowing for:
/// - compression
/// - chunked transfer
/// - encryption
/// - custom flags
#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct PacketOptions {
    /// Whether the packet should be compressed.
    pub compress: bool,
    /// The sequence number of the packet (for chunked transfer).
    pub sequence: Option<usize>,
    /// The total number of chunks in the packet (for chunked transfer).
    pub total_chunks: Option<usize>,
    /// Whether the packet is encrypted.
    pub encrypted: bool,
    /// Custom flags for protocol extensions.
    pub custom_flags: Option<CustomFlags>,
}

bitflags! {
    /// Custom protocol flags for PacketOptions.
    pub struct CustomFlags: u16 {
        /// High priority packet.
        const PRIORITY      = 0b0000_0001;
        /// Reliable delivery (requires acknowledgment).
        const RELIABLE      = 0b0000_0010;
        /// Broadcast to multiple recipients.
        const BROADCAST     = 0b0000_0100;
        /// Reserved for future use.
        const RESERVED      = 0b0000_1000;
        /// Packet is a fragment (for manual chunking).
        const FRAGMENT      = 0b0001_0000;
        /// Packet requests a response (like an ACK).
        const REQUEST_ACK   = 0b0010_0000;
        /// Packet is for tracing/debugging.
        const TRACE         = 0b0100_0000;
        /// Packet is encrypted with a custom scheme.
        const CUSTOM_ENCRYPT= 0b1000_0000;
        // Add more as needed for your protocol
    }
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

    /// Sets chunking information for the packet.
    pub fn with_chunking(mut self, sequence: usize, total_chunks: usize) -> Self {
        self.sequence = Some(sequence);
        self.total_chunks = Some(total_chunks);
        self
    }

    /// Enables encryption for the packet.
    pub fn with_encryption(mut self) -> Self {
        self.encrypted = true;
        self
    }

    /// Sets custom flags for protocol extensions.
    pub fn with_custom_flags(mut self, flags: u16) -> Self {
        self.custom_flags = Some(flags);
        self
    }

    pub fn encode(self, supports_binary: bool) -> RawData {
        if supports_binary {
            let bytes = bincode::serialize(&self).unwrap();
            RawData::Binary(bytes)
        } else {
            let text = self::encode_compact(&self);
            RawData::Text(text)
        }
    }

    pub fn decode(data: RawData, supports_binary: bool) -> Result<Self, bincode::Error> {
        match data {
            RawData::Binary(bytes) if supports_binary => {
                bincode::deserialize(&bytes)?
            },
            RawData::Text(text) => {
                let decoded = self::decode_compact(&text);
                if decoded.is_none() {
                    return Err(bincode::Error::new(bincode::ErrorKind::Custom("Failed to decode compact text".into())));
                }
                Ok(decoded)
            }
        }
    }

        /// Encodes PacketOptions as a compact string (e.g., "1:42:100:0:7").
    pub fn encode_compact(&self) -> String {
        // Bitmask for which Option fields are present
        let mut mask = 0u8;
        if self.sequence.is_some() { mask |= 0b0001; }
        if self.total_chunks.is_some() { mask |= 0b0010; }
        if self.custom_flags.is_some() { mask |= 0b0100; }

        format!(
            "{}:{}:{}:{}:{}:{}",
            self.compress as u8,
            self.encrypted as u8,
            mask,
            self.sequence.unwrap_or(0),
            self.total_chunks.unwrap_or(0),
            self.custom_flags.unwrap_or(0)
        )
    }

    /// Decodes PacketOptions from a compact string.
    pub fn decode_compact(s: &str) -> Option<Self> {
        let parts: Vec<_> = s.split(':').collect();
        if parts.len() != 6 { return None; }
        let compress = parts[0] == "1";
        let encrypted = parts[1] == "1";
        let mask = parts[2].parse::<u8>().ok()?;
        let sequence = if mask & 0b0001 != 0 { Some(parts[3].parse().ok()?) } else { None };
        let total_chunks = if mask & 0b0010 != 0 { Some(parts[4].parse().ok()?) } else { None };
        let custom_flags = if mask & 0b0100 != 0 { Some(parts[5].parse().ok()?) } else { None };

        Some(PacketOptions {
            compress,
            sequence,
            total_chunks,
            encrypted,
            custom_flags,
        })
    }
}