use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Stream;
use pin_project::pin_project;

use crate::packet::Packet;
use crate::constants::{RawData, BINARY_MASK, PLAIN_TEXT_MASK, BinaryType};

#[pin_project]
pub struct PacketEncoderStream<S> {
    #[pin]
    stream: S,
}

impl<S> PacketEncoderStream<S>
where
    S: Stream<Item = Packet>,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
}

impl<S> Stream for PacketEncoderStream<S>
where
    S: Stream<Item = Packet>,
{
    type Item = BinaryType;

    fn poll_next(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.stream.poll_next(ctx) {
            Poll::Ready(Some(packet)) => {
                let encoded_packet = packet.encode_binary();
                let mut result: BinaryType = Vec::new();

                // Add length header similar to WebSocket-like encoding
                let payload_length = encoded_packet.len();
                let mut header: BinaryType = Vec::new();
                if payload_length < 126 {
                    header.push(payload_length as u8);
                } else if payload_length < 65536 {
                    header.push(126);
                    header.push((payload_length >> 8) as u8);
                    header.push(payload_length as u8);
                } else {
                    header.push(127);
                    header.push((payload_length >> 56) as u8);
                    header.push((payload_length >> 48) as u8);
                    header.push((payload_length >> 40) as u8);
                    header.push((payload_length >> 32) as u8);
                    header.push((payload_length >> 24) as u8);
                    header.push((payload_length >> 16) as u8);
                    header.push((payload_length >> 8) as u8);
                    header.push(payload_length as u8);
                }
                header[0] |= match &packet.data {
                    Some(RawData::Binary(..))=> { BINARY_MASK },
                    Some(RawData::Text(..)) => { PLAIN_TEXT_MASK },
                    None => { PLAIN_TEXT_MASK }
                };
                result.extend(header);
                result.extend(encoded_packet);

                Poll::Ready(Some(result))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
