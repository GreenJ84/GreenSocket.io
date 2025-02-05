use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Stream;
use pin_project::pin_project;
use crate::BinaryType;
use crate::constants::{RawData, BINARY_MASK, PLAIN_TEXT_MASK};
use crate::packet::Packet;

#[pin_project]
pub struct PacketDecoderStream<S> {
    #[pin]
    stream: S,
    chunks: VecDeque<Vec<u8>>,
    state: State,
    expected_length: usize,
    is_binary: bool,
}

#[derive(Debug, Clone, Copy)]
enum State {
    ReadHeader,
    ReadExtendedLength16,
    ReadExtendedLength64,
    ReadPayload,
}

impl<S> PacketDecoderStream<S>
where
    S: Stream<Item = BinaryType>,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            chunks: VecDeque::new(),
            state: State::ReadHeader,
            expected_length: 0,
            is_binary: false,
        }
    }

}

fn total_length(chunks: &VecDeque<BinaryType>) -> usize {
    chunks.iter().map(|chunk| chunk.len()).sum()
}

fn concat_chunks(chunks: &mut VecDeque<BinaryType>, size: usize) -> BinaryType {
    let mut result = Vec::with_capacity(size);
    while result.len() < size {
        if let Some(chunk) = chunks.pop_front() {
            result.extend_from_slice(&chunk);
        }
    }
    result
}

impl<S> Stream for PacketDecoderStream<S>
where
    S: Stream<Item = BinaryType>,
{
    type Item = Packet;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        match this.stream.poll_next(cx) {
            Poll::Ready(Some(chunk)) => {
                this.chunks.push_back(chunk);

                loop {
                    match this.state {
                        State::ReadHeader => {
                            if total_length(this.chunks) < 1 {
                                break Poll::Pending;
                            }
                            let header = concat_chunks(this.chunks, 1);
                            *this.is_binary = (header[0] & BINARY_MASK) != PLAIN_TEXT_MASK;
                            *this.expected_length = (header[0] & 0x7f) as usize;
                            if *this.expected_length < 126 {
                                *this.state = State::ReadPayload;
                            } else if *this.expected_length == 126 {
                                *this.state = State::ReadExtendedLength16;
                            } else {
                                *this.state = State::ReadExtendedLength64;
                            }
                        }
                        State::ReadExtendedLength16 => {
                            if total_length(this.chunks) < 2 {
                                break Poll::Pending;
                            }
                            let header = concat_chunks(this.chunks, 2);
                            *this.expected_length = ((header[0] as usize) << 8) | (header[1] as usize);
                            *this.state = State::ReadPayload;
                        }
                        State::ReadExtendedLength64 => {
                            if total_length(this.chunks) < 8 {
                                break Poll::Pending;
                            }
                            let header = concat_chunks(this.chunks, 8);
                            *this.expected_length = (header[0] as usize) << 56
                                | (header[1] as usize) << 48
                                | (header[2] as usize) << 40
                                | (header[3] as usize) << 32
                                | (header[4] as usize) << 24
                                | (header[5] as usize) << 16
                                | (header[6] as usize) << 8
                                | header[7] as usize;
                            *this.state = State::ReadPayload;
                        }
                        State::ReadPayload => {
                            if total_length(this.chunks) < *this.expected_length {
                                break Poll::Pending;
                            }
                            let payload = concat_chunks(this.chunks, *this.expected_length);
                            let decoded_packet = Packet::decode(RawData::Binary(payload));
                            break Poll::Ready(Some(decoded_packet));
                        }
                    }
                }
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
