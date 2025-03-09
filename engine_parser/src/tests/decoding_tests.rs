use base64::Engine;
use base64::engine::general_purpose;

use crate::{*, packet::*, constants::*};


#[cfg(test)]
mod single_packet_decoding_tests {
    use base64::Engine;
    use base64::engine::general_purpose;
    use super::*;

    #[test]
    fn decode_empty_binary_packet() {
        let encoded_data = RawData::Binary(vec![]);

        let packet = Packet::decode(encoded_data);
        assert_eq!(packet._type, PacketType::Noop);
        assert_eq!(packet.data, None);
    }

    #[test]
    fn decode_empty_text_packet() {
        let encoded_data = RawData::Text(String::new());

        let packet = Packet::decode(encoded_data);
        assert_eq!(packet._type, PacketType::Noop);
        assert_eq!(packet.data, None);
    }

    #[test]
    fn decode_no_data_binary_packet() {
        let encoded_data = RawData::Binary(vec![BINARY_MASK, '3' as u8]);

        let packet = Packet::decode(encoded_data);
        assert_eq!(packet._type, PacketType::Pong);
        assert_eq!(packet.data, None);
    }

    #[test]
    fn decode_no_data_text_packet() {
        let encoded_data = RawData::Text(String::from("5"));

        let packet = Packet::decode(encoded_data);
        assert_eq!(packet._type, PacketType::Upgrade);
        assert_eq!(packet.data, None);
    }


    #[test]
    fn decode_text_encoded_binary_packet() {
        let encoded_data = RawData::Text(
            format!("b{}{}", PacketType::Close.as_char(), general_purpose::URL_SAFE.encode(vec![0x02, 0x00, 0x00]))
        );

        let packet = Packet::decode(encoded_data);
        if let Some(RawData::Binary(data)) = &packet.data {
            assert_eq!(data.clone(), vec![0x02, 0x00, 0x00]);
        } else { assert!(false, "Expected Binary Packet data") }

        assert_eq!(packet.data, Some(RawData::Binary(vec![0x02, 0x00, 0x00])));
        assert_eq!(packet._type, PacketType::Close);
    }

    #[test]
    fn decode_text_encoded_text_packet() {
        let encoded_data = RawData::Text(String::from("4Hello, world!"));

        let packet = Packet::decode(encoded_data);
        assert_eq!(packet._type, PacketType::Message);
        assert_eq!(packet.data, Some(RawData::Text(String::from("Hello, world!"))));
    }

    #[test]
    fn decode_binary_encoded_binary_packet() {
        let encoded_data = RawData::Binary(vec![BINARY_MASK, '0' as u8, 0x02, 0x00, 0x00]);

        let packet = Packet::decode(encoded_data);
        assert_eq!(packet._type, PacketType::Open);
        assert_eq!(packet.data, Some(RawData::Binary(vec![0x02, 0x00, 0x00])));
    }

    #[test]
    fn decode_binary_encoded_text_packet() {
        let mut binary = vec![PLAIN_TEXT_MASK, 'e' as u8];
        binary.extend_from_slice("501".as_bytes());
        let encoded_data = RawData::Binary(binary);

        let packet = Packet::decode(encoded_data);
        assert_eq!(packet._type, PacketType::Error);
        assert_eq!(packet.data, Some(RawData::Text(String::from("501"))));
    }

    #[test]
    fn decode_all_packet_types(){
        for t in 0..8 {
            let packet_char = if t == 8 { char::from_u32(t as u32).unwrap() as u8 } else { 'e' as u8 };
            let encoded_data = RawData::Binary(vec![
                BINARY_MASK,
                packet_char,
                0x44, 0x44, 0x44
            ]);

            let packet = Packet::decode(encoded_data);
            assert_eq!(
                packet._type,
                PacketType::from_char(char::from(packet_char)).unwrap()
            );
            assert_eq!(packet.data, Some(RawData::Binary(vec![0x44, 0x44, 0x44])));
        }

    }
}

#[cfg(test)]
mod payload_decoding_tests{
    use super::*;

    fn binary(i: u32, bin: BinaryType) -> BinaryType {
        let mut data = vec![BINARY_MASK, char::from_digit(i % 6, 10).unwrap() as u8];
        data.extend(bin);
        data.push(SEPARATOR_BYTE);
        data
    }
    fn binary_as_text(i: u32, bin: BinaryType) -> String {
        let mut data = format!("b{}", char::from_digit(i % 6, 10).unwrap());
        data.push_str(&general_purpose::URL_SAFE.encode(bin));
        data.push(SEPARATOR_BYTE as char);
        data
    }

    fn text_as_binary(i: u32, text: &str) -> BinaryType {
        let mut data = vec![PLAIN_TEXT_MASK, char::from_digit(i % 6, 10).unwrap() as u8];
        data.extend(text.as_bytes());
        data.push(SEPARATOR_BYTE);
        data
    }
    fn text(i: u32, text: &str) -> String {
        format!("{}{}{}", char::from_digit(i % 6, 10).unwrap(), text, SEPARATOR_BYTE as char)
    }


    #[test]
    fn decode_empty_payload() {
        assert_eq!(
            Packet::decode_payload(RawData::Text(String::new())),
            Vec::new()
        );
    }

    #[test]
    fn decode_payload_with_no_data_binary_packets() {
        let payload = (0..20)
            .map(|i| binary(i, Vec::new()))
            .flatten()
            .collect::<BinaryType>();

        let decoded_payload = Packet::decode_payload(RawData::Binary(payload));
        for i in 0..20u32 {
            assert_eq!(
                decoded_payload[i as usize],
                Packet::new(
                    PacketType::from_char(char::from_digit(i % 6, 10).unwrap()).unwrap(),
                    None,
                    None
                ),
                "Packet {} has failed. {:?}", i, decoded_payload[i as usize]
            )
        }
    }

    #[test]
    fn decode_payload_with_no_data_text_packets() {
        let payload = (0..20)
            .map(|i| text(i, ""))
            .collect::<Vec<String>>()
            .join("");
        let decoded_payload = Packet::decode_payload(RawData::Text(payload));
        for i in 0..20u32 {
            assert_eq!(
                decoded_payload[i as usize],
                Packet::new(
                    PacketType::from_char(char::from_digit(i % 6, 10).unwrap()).unwrap(),
                    None,
                    None
                ),
                "Packet {} has failed. {:?}", i, decoded_payload[i as usize]
            )
        }
    }

    #[test]
    fn decode_payload_with_binary_packets() {
        let payload = (0..20)
            .map(|i| binary_as_text(i, Vec::from("Hello".as_bytes())))
            .collect::<Vec<String>>()
            .join("");
        let decoded_payload = Packet::decode_payload(RawData::Text(payload));
        for i in 0..20u32 {
            assert_eq!(
                decoded_payload[i as usize],
                Packet::new(
                    PacketType::from_char(char::from_digit(i % 6, 10).unwrap()).unwrap(),
                    None,
                    Some(RawData::Binary(Vec::from("Hello".as_bytes())))
                ),
                "Packet {} has failed. {:?}", i, if let Some(RawData::Binary(d)) = &decoded_payload[i as usize].data { String::from_utf8(d.clone()).unwrap()} else { "None".into() }
            )
        }
    }

    #[test]
    fn decode_payload_with_text_packets() {
        let payload = (0..20)
            .map(|i|  text(i, "Hello"))
            .collect::<Vec<String>>()
            .join("");
        let decoded_payload = Packet::decode_payload(RawData::Text(payload));
        for i in 0..20u32 {
            assert_eq!(
                decoded_payload[i as usize],
                Packet::new(
                    PacketType::from_char(char::from_digit(i % 6, 10).unwrap()).unwrap(),
                    None,
                    Some(RawData::Text("Hello".into()))
                ),
                "Packet {} has failed. {:?}", i, if let Some(RawData::Binary(d)) = &decoded_payload[i as usize].data { String::from_utf8(d.clone()).unwrap()} else { "None".into() }
            )
        }
    }

    #[test]
    fn decode_payload_packets_with_mixed_data_packets() {
        let payload = (0..20)
            .map(|i: u32|
                if i % 2 == 0 {
                    binary_as_text(i, "Binary".as_bytes().into())
                } else {
                    text(i, "Text")
                }
            )
            .collect::<Vec<String>>()
            .join("");
        let decoded_payload = Packet::decode_payload(RawData::Text(payload));

        for i in 0..20u32 {
            assert_eq!(
                decoded_payload[i as usize],
                Packet::new(
                    PacketType::from_char(char::from_digit(i % 6, 10).unwrap()).unwrap(),
                    None,
                    if i % 2== 0 {
                        Some(RawData::Binary(Vec::from("Binary".as_bytes())))
                    } else {
                        Some(RawData::Text("Text".into()))
                    }
                ),
                "Packet {} has failed. {:?}", i, if let Some(RawData::Binary(d)) = &decoded_payload[i as usize].data { d.clone() } else { "None".into() }
            )
        }
    }

    #[test]
    fn decode_payload_packets_with_large_data_binary_packets() {
        let payload = (0..20)
            .map(|i|  binary(i, vec![0x44].repeat(i16::MAX as usize)))
            .flatten()
            .collect::<BinaryType>();
        let decoded_payload = Packet::decode_payload(RawData::Binary(payload));
        for i in 0..20u32 {
            assert_eq!(
                decoded_payload[i as usize],
                Packet::new(
                    PacketType::from_char(char::from_digit(i % 6, 10).unwrap()).unwrap(),
                    None,
                    Some(RawData::Binary(vec![0x44].repeat(i16::MAX as usize)))
                ),
                "Packet {} has failed. {:?}", i, if let Some(RawData::Binary(d)) = &decoded_payload[i as usize].data { String::from_utf8(d.clone()).unwrap()} else { "None".into() }
            )
        }
    }

    #[test]
    fn decode_payload_packets_with_large_data_text_packets() {
        let payload = (0..20)
            .map(|i|  text(i, &"Hello".repeat(i8::MAX as usize)))
            .collect::<Vec<String>>()
            .join("");
        let decoded_payload = Packet::decode_payload(RawData::Text(payload));
        for i in 0..20u32 {
            assert_eq!(
                decoded_payload[i as usize],
                Packet::new(
                    PacketType::from_char(char::from_digit(i % 6, 10).unwrap()).unwrap(),
                    None,
                    Some(RawData::Text("Hello".repeat(i8::MAX as usize)))
                ),
                "Packet {} has failed. {:?}", i, if let Some(RawData::Binary(d)) = &decoded_payload[i as usize].data { String::from_utf8(d.clone()).unwrap()} else { "None".into() }
            )
        }
    }

    #[test]
    fn decode_payload_packets_with_large_data_mixed_packets() {
        let payload = (0..20)
            .map(|i: u32|
            if i % 2 == 0 {
                binary_as_text(i, vec![0x44].repeat(i16::MAX as usize))
            } else {
                text(i, &"Hello".repeat(i8::MAX as usize))
            }
            )
            .collect::<Vec<String>>()
            .join("");
        let decoded_payload = Packet::decode_payload(RawData::Text(payload));

        for i in 0..20u32 {
            assert_eq!(
                decoded_payload[i as usize],
                Packet::new(
                    PacketType::from_char(char::from_digit(i % 6, 10).unwrap()).unwrap(),
                    None,
                    if i % 2== 0 {
                        Some(RawData::Binary(vec![0x44].repeat(i16::MAX as usize)))
                    } else {
                        Some(RawData::Text("Hello".repeat(i8::MAX as usize)))
                    }
                ),
                "Packet {} has failed. {:?}", i, if let Some(RawData::Binary(d)) = &decoded_payload[i as usize].data { String::from_utf8(d.clone()).unwrap()} else { "None".into() }
            )
        }
    }

    #[test]
    fn decode_large_mixed_payload() {
        let payload = (0..i16::MAX as u32)
            .map(|i|
            if i % 2 == 0 {
                binary_as_text(i, vec![0x44])
            } else {
                text(i, &"Hello")
            }
            )
            .collect::<Vec<String>>()
            .join("");
        let decoded_payload = Packet::decode_payload(RawData::Text(payload));

        for i in 0..i16::MAX as u32 {
            assert_eq!(
                decoded_payload[i as usize],
                Packet::new(
                    PacketType::from_char(char::from_digit(i % 6, 10).unwrap()).unwrap(),
                    None,
                    if i % 2== 0 {
                        Some(RawData::Binary(vec![0x44]))
                    } else {
                        Some(RawData::Text("Hello".into()))
                    }
                ),
                "Packet {} has failed. {:?}", i, if let Some(RawData::Binary(d)) = &decoded_payload[i as usize].data { String::from_utf8(d.clone()).unwrap()} else { "None".into() }
            )
        }
    }
}

#[cfg(test)]
mod decoding_stream_tests{
    use super::*;
    use futures::{stream, Stream, StreamExt};
    use tokio::sync::mpsc;

    fn build_header(binary: bool, data_length: usize) -> BinaryType {
        let mut header: BinaryType = Vec::new();
        if data_length < 126 {
            header.push(data_length as u8);
        } else if data_length < 65536 {
            header.push(126);
            header.push((data_length >> 8) as u8);
            header.push(data_length as u8);
        } else {
            header.push(127);
            header.push((data_length >> 56) as u8);
            header.push((data_length >> 48) as u8);
            header.push((data_length >> 40) as u8);
            header.push((data_length >> 32) as u8);
            header.push((data_length >> 24) as u8);
            header.push((data_length >> 16) as u8);
            header.push((data_length >> 8) as u8);
            header.push(data_length as u8);
        }
        if binary {
            header[0] |= BINARY_MASK;
        }
        header
    }
    fn build_bin_packets(count: u32, data_length: usize) -> Vec<BinaryType> {
        (0..count)
            .map(|i| {
                let mut v = build_header(true, data_length + 2);
                v.extend([
                    BINARY_MASK,
                    char::from_digit(i % 6, 10).unwrap() as u8
                ]);
                v.extend([0x44u8].repeat(data_length));
                v
            })
            .collect::<Vec<BinaryType>>()
    }
    fn build_text_packets(count: u32, data_length: usize) -> Vec<BinaryType>{
        (0..count)
            .map(|i| {
                let mut v = build_header(false, data_length + 2);
                v.extend([
                    PLAIN_TEXT_MASK,
                    char::from_digit(i % 6, 10).unwrap() as u8
                ]);
                v.extend("G".as_bytes().repeat(data_length));
                v
            })
            .collect::<Vec<BinaryType>>()
    }
    fn build_stream(count: u32, data_length: usize, mixed: bool, binary: bool) -> PacketDecoderStream<impl Stream<Item = BinaryType>> {
        let stream ;
        if mixed {
            let bin = build_bin_packets(count, data_length);
            let text = build_text_packets(count, data_length);

            let mut mix = Vec::<BinaryType>::new();
            for i in  0..count {
                mix.push(if i % 2 == 0 {
                    bin[i as usize].clone()
                } else {
                    text[i as usize].clone()
                })
            }
            stream = stream::iter(mix);
        } else if binary {
            stream = stream::iter(build_bin_packets(count, data_length));
        } else {
            stream = stream::iter(build_text_packets(count, data_length));
        }
        PacketDecoderStream::new(stream)
    }

    #[tokio::test]
    async fn decoding_stream_functions_with_empty_binary_packet_streams() {
        let mut decoding_stream = build_stream(20, 0, false, true);

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data, None);
            count+= 1;
            if count == 20 {
                break;
            }
        }
    }

    #[tokio::test]
    async fn decoding_stream_functions_with_empty_text_packet_streams() {
        let mut decoding_stream = build_stream(20, 0, false, false);

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data, None);
            count+= 1;
            if count == 20 {
                break;
            }
        }
    }

    #[tokio::test]
    async fn decoding_stream_functions_with_empty_mixed_packet_streams() {
        let mut decoding_stream = build_stream(20, 0, true, false);

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data, None);
            count+= 1;
            if count == 20 {
                break;
            }
        }
    }

    #[tokio::test]
    ///! TODO: Improve stream processing. Hangups with data over 2_000 bytes.
    async fn decoding_stream_functions_with_large_binary_packet_streams() {
        let mut decoding_stream = build_stream(20, 2_000usize, false, true);

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data, Some(RawData::Binary([0x44u8].repeat(2_000usize))));
            count+= 1;
            if count == 20 {
                break;
            }
        }
    }

    #[tokio::test]
    async fn decoding_stream_functions_with_large_text_packet_streams() {
        let mut decoding_stream = build_stream(20, 2_000usize, false, false);

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data, Some(RawData::Text("G".repeat(2_000usize))));
            count+= 1;
            if count == 20 {
                break;
            }
        }
    }

    #[tokio::test]
    async fn decoding_stream_functions_with_large_mixed_packet_streams() {
        let mut decoding_stream = build_stream(20, 2_000usize, true, false);

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data,
                if count % 2 == 0 {
                   Some(RawData::Binary([0x44u8].repeat(2_000usize)))
                } else {
                   Some(RawData::Text("G".repeat(2_000usize)))
                }
            );
            count+= 1;
            if count == 20 {
                break;
            }
        }
    }

    #[tokio::test]
    async fn decoding_stream_functions_with_large_streams_of_binary_packets() {
        let mut decoding_stream = build_stream(i16::MAX as u32, 200usize, false, true);

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data, Some(RawData::Binary([0x44u8].repeat(200usize))));
            count+= 1;
            if count == i16::MAX as u32 {
                break;
            }
        }
    }

    #[tokio::test]
    async fn decoding_stream_functions_with_large_streams_of_text_packets() {
        let mut decoding_stream = build_stream(2_000u32, 200usize, false, false);

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data, Some(RawData::Text("G".repeat(200usize))));
            count+= 1;
            if count == 2_000u32 {
                break;
            }
        }
    }

    #[tokio::test]
    async fn decoding_stream_functions_with_large_streams_of_mixed_packets() {
        let mut decoding_stream = build_stream(2_000u32, 200usize, true, false);

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data,
               if count % 2 == 0 {
                   Some(RawData::Binary([0x44u8].repeat(200usize)))
               } else {
                   Some(RawData::Text("G".repeat(200usize)))
               }
            );
            count+= 1;
            if count == 2_000u32 {
                break;
            }
        }
    }

    #[tokio::test]
    async fn decoding_binary_packets_with_async_stream_channel() {
        let (tx, rx) = mpsc::channel::<BinaryType>(200);
        let mut decoding_stream = PacketDecoderStream::new(tokio_stream::wrappers::ReceiverStream::new(rx));

        // Spawn a task to send data asynchronously
        tokio::spawn(async move {
            let mut packets = build_bin_packets(200, 100);
            for _ in 0..packets.len() {
                tx.send(packets.remove(0)).await.unwrap();
            }
        });

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data, Some(RawData::Binary([0x44u8].repeat(100usize))));
            count += 1;
        }

        assert_eq!(count, 200, "Should have received all 200 packets");
    }

    #[tokio::test]
    async fn decoding_text_packets_with_async_stream_channel() {
        let (tx, rx) = mpsc::channel::<BinaryType>(200);
        let mut decoding_stream = PacketDecoderStream::new(tokio_stream::wrappers::ReceiverStream::new(rx));

        // Spawn a task to send data asynchronously
        tokio::spawn(async move {
            let mut packets = build_text_packets(200, 100);
            for _ in 0..packets.len() {
                tx.send(packets.remove(0)).await.unwrap();
            }
        });

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data, Some(RawData::Text("G".repeat(100usize))));
            count += 1;
        }

        assert_eq!(count, 200, "Should have received all 200 packets");
    }

    #[tokio::test]
    async fn decoding_high_throughput_with_async_channel() {
        let (tx, rx) = mpsc::channel::<BinaryType>(200);
        let mut decoding_stream = PacketDecoderStream::new(tokio_stream::wrappers::ReceiverStream::new(rx));

        // Spawn a task to send data asynchronously
        tokio::spawn(async move {
            let mut packets = build_bin_packets(20_000, 20_000);
            for _ in 0..packets.len() {
                if tx.send(packets.remove(0)).await.is_err() {
                    eprintln!("Receiver dropped before sending all packets!");
                    break;
                }
            }
        });

        let mut count = 0;
        while let Some(packet) = decoding_stream.next().await {
            assert_eq!(packet._type, PacketType::from_char(char::from_digit(count % 6, 10).unwrap()).unwrap());
            assert_eq!(packet.data, Some(RawData::Binary([0x44u8].repeat(20_000usize))));
            count += 1;
        }

        assert_eq!(count, 20_000, "Should have received all 200 packets");
    }

}
