mod constants;
mod packet;
mod parser;

#[cfg(test)]
mod tests;

pub use packet::{
    Packet,
    MAX_PACKET_SIZE,
    types::PacketType,
    options::PacketOptions,
    error::PacketError,
};
pub use parser::{
    error::{DecodingError, EncodingError},
//     encoding_stream::PacketEncoderStream,
//     decoding_stream::PacketDecoderStream,
};
pub use constants::{
    BinaryType,
    RawData
};