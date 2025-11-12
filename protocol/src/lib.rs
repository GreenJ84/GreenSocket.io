mod constants;
mod decoding;
mod encoding;
mod error;
mod packet;

#[cfg(test)]
mod tests;

pub use error::{DecodingError, EncodingError};
pub use packet::{
    error::PacketError, options::PacketOptions, types::PacketType, Packet, MAX_PACKET_SIZE,
};

// encoding_stream::PacketEncoderStream,
// decoding_stream::PacketDecoderStream,

pub use constants::{BinaryType, RawData};
