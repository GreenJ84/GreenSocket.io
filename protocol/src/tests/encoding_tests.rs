use base64::Engine;
use base64::engine::general_purpose;

use crate::{*, constants::*};

fn build_packet(packet_type: PacketType, data: Option<RawData>) -> Packet{
    Packet::new(
        packet_type,
        Some(PacketOptions { compress: false }),
        data,
    )
}

#[cfg(test)]
mod single_packet_encoding_tests {
    use super::*;

    #[test]
    fn encode_binary_packet_with_binary_support() {
        let packet = build_packet(PacketType::Message, Some(RawData::Binary(vec![1, 2, 3, 4, 5])));

        let encoded = packet.encode(true);
        match encoded {
            RawData::Binary(data) => {
                assert_eq!(data, vec![BINARY_MASK, PacketType::Message.as_char() as u8, 1, 2, 3, 4, 5])
            },
            _ => panic!("Expected Binary data"),
        }
    }

    #[test]
    fn encode_binary_packet_without_binary_support() {
        let packet = build_packet(PacketType::Message, Some(RawData::Binary(vec![1, 2, 3, 4, 5])));

        let encoded = packet.encode(false);
        match encoded {
            RawData::Text(text) => {
                assert_eq!(text, format!("b4{}", general_purpose::URL_SAFE.encode(vec![1, 2, 3, 4, 5]))); // base64 encoded data
            }
            _ => panic!("Expected Text data"),
        }
    }

    #[test]
    fn encode_text_data_with_binary_support() {
        let packet = build_packet(PacketType::Message, Some(RawData::Text("Hello, world!".to_string())));

        let encoded = packet.encode(true);
        match encoded {
            RawData::Text(text) => {
                assert_eq!(text, "4Hello, world!");
            }
            _ => panic!("Expected Text data"),
        }
    }

    #[test]
    fn encode_text_data_without_binary_support() {
        let packet = build_packet(PacketType::Message, Some(RawData::Text("Hello, world!".to_string())));

        let encoded = packet.encode(false);
        match encoded {
            RawData::Text(text) => {
                assert_eq!(text, "4Hello, world!");
            }
            _ => panic!("Expected Text data"),
        }
    }

    #[test]
    fn encode_no_data() {
        let packet = build_packet(PacketType::Ping, None);

        let encoded = packet.encode(true);
        match encoded {
            RawData::Text(text) => {
                assert_eq!(text, "2");
            }
            _ => panic!("Expected Text data"),
        }
    }
}

#[cfg(test)]
mod forced_binary_encoding_tests{
    use super::*;

    #[test]
    fn force_binary_encode_binary_data() {
        let packet = build_packet(PacketType::Message, Some(RawData::Binary(vec![1, 2, 3, 4, 5])));

        let encoded = packet.encode_binary();
        assert_eq!(encoded, vec![BINARY_MASK, PacketType::Message.as_char() as u8, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn force_binary_encode_text_data() {
        let packet = build_packet(PacketType::Message, Some(RawData::Text("Hello world!".into())));

        let encoded = packet.encode_binary();
        let mut expected = Vec::from([PLAIN_TEXT_MASK]);
        expected.extend_from_slice(&String::from("4Hello world!").into_bytes());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn force_binary_encode_no_data() {
        let packet = build_packet(PacketType::Open, None);

        let encoded = packet.encode_binary();
        let mut expected = Vec::from([PLAIN_TEXT_MASK]);
        expected.extend_from_slice(&String::from("0").into_bytes());
        assert_eq!(encoded, expected);
    }
}

#[cfg(test)]
mod payload_encoding_tests{
    use super::*;

    #[test]
    fn encode_payload_packets_with_no_data() {
        let packet1 = build_packet(PacketType::Ping, None);
        let packets = (0..20).map(|_| packet1.clone()).collect::<Vec<Packet>>();

        let encoded_payload = Packet::encode_payload(&packets, false);

        // Should be a
        let mut expected_payload = String::new();
        for _ in 0..20{
            expected_payload.push_str(&format!("2{}", SEPARATOR_BYTE as char));
        }
        expected_payload.pop();
        assert_eq!(encoded_payload, RawData::Text(expected_payload));
    }


    #[test]
    fn encode_payload_packets_with_mixed_data() {
        let packet1 = build_packet(PacketType::Message, Some(RawData::Binary(vec![1, 2, 3])));
        let packet2 = build_packet(PacketType::Ping, Some(RawData::Text("Ping".to_string())));

        let mut packets = vec![];
        for _ in 0..20 { packets.extend_from_slice(&vec![packet1.clone(), packet2.clone()]) }

        let encoded_payload = Packet::encode_payload(&packets, false);

        let mut expected_payload= String::new();
        for _ in 0..20{
            // Packet 1
            expected_payload.push_str("b4");
            expected_payload.push_str(&general_purpose::URL_SAFE.encode(vec![1,2,3]));
            expected_payload.push(SEPARATOR_BYTE as char);
            // Packet 2
            expected_payload.push(packet2._type.as_char());
            expected_payload.push_str("Ping");
            expected_payload.push(SEPARATOR_BYTE as char);
        }
        expected_payload.pop();

        assert_eq!(encoded_payload, RawData::Text(expected_payload));
    }

    #[test]
    fn encode_payload_packets_with_large_data() {
        let data = (0..i16::MAX).map(|_| 0x44).collect::<Vec<u8>>();
        let packet1 = build_packet(PacketType::Ping, Some(RawData::Binary(data.clone())));
        let packets = (0..20).map(|_| packet1.clone()).collect::<Vec<Packet>>();

        let encoded_payload = Packet::encode_payload(&packets, true);

        let mut expected_payload = Vec::new();
        for _ in 0..20{
            expected_payload.extend([BINARY_MASK, '2' as u8]);
            expected_payload.extend(data.clone());
            expected_payload.push(SEPARATOR_BYTE);
        }
        expected_payload.pop();
        assert_eq!(encoded_payload, RawData::Binary(expected_payload));
    }

    #[test]
    fn encode_large_payload() {
        let data = (0..20).map(|_| 0x44).collect::<Vec<u8>>();
        let packet1 = build_packet(PacketType::Ping, Some(RawData::Binary(data.clone())) );
        let packets = (0..i16::MAX).map(|_| packet1.clone()).collect::<Vec<Packet>>();

        let encoded_payload = Packet::encode_payload(&packets, false);

        let mut expected_payload = String::new();
        for _ in 0..i16::MAX{
            expected_payload.push_str("b2");
            expected_payload.push_str(&general_purpose::URL_SAFE.encode(data.clone()));
            expected_payload.push(SEPARATOR_BYTE as char);
        }
        expected_payload.pop();
        assert_eq!(encoded_payload, RawData::Text(expected_payload));
    }



    #[test]
    fn encode_empty_payload() {
        let packets: Vec<Packet> = Vec::new();
        let encoded_payload = Packet::encode_payload(&packets, true);

        assert_eq!(encoded_payload.len(), 0);
    }
}

#[cfg(test)]
mod encoding_stream_tests{
    use super::*;
    use futures::{stream, StreamExt};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn encoding_stream_functions_with_packets() {
        let packets= (0..20)
            .map(|_| Packet::new(
                PacketType::Message, None, Some(RawData::Text("Hello".into()))
            ))
            .collect::<Vec<Packet>>();
        let mut encoding_stream = PacketEncoderStream::new(
            stream::iter(packets)
        );

        let mut count = 0;
        while let Some(encoded) = encoding_stream.next().await {
            assert!(!encoded.is_empty(), "Encoded packet should not be empty");
            assert_eq!(encoded[0], 7, "First byte should be 7 -> (Mask + Category byte + 5 bytes for Hello");
            assert_eq!(encoded[1], PLAIN_TEXT_MASK, "Second byte should be the text mask");
            assert_eq!(&encoded[2..], "4Hello".as_bytes(), "Last bytes should be \"Hello\" as bytes");
            count += 1;
            if count == 20 { break; }
        }
        assert_eq!(count, 20, "There should be twenty encoded packets");
    }

    // Test encoding stream functions with empty data
    #[tokio::test]
    async fn encoding_stream_functions_with_empty_data_packets() {
        let packets= (0..20)
            .map(|_| Packet::new(
                PacketType::Ping, None, None
            ))
            .collect::<Vec<Packet>>();
        let mut encoding_stream = PacketEncoderStream::new(
            stream::iter(packets)
        );

        let mut count = 0;
        while let Some(encoded) = encoding_stream.next().await {
            assert!(!encoded.is_empty(), "Encoded packet should not be empty");
            assert_eq!(encoded[0], 2, "First byte should be 2 (Mask & Identifier)");
            assert_eq!(encoded[1], PLAIN_TEXT_MASK, "Second byte should be the text mask");
            assert_eq!(encoded[2], 50, "Third byte should be '2' as a byte");
            count += 1;
            if count == 20 { break; }
        }
        assert_eq!(count, 20, "There should be twenty encoded packets");
    }

    // Test encoding stream with large data
    #[tokio::test]
    async fn encoding_stream_functions_with_large_data_packets() {
        let packets= (0..20)
            .map(|_| Packet::new(
                PacketType::Message,
                None,
                Some(RawData::Binary((0..i16::MAX).map(|_| 0x44).collect::<Vec<u8>>()))
            ))
            .collect::<Vec<Packet>>();
        let mut encoding_stream = PacketEncoderStream::new(
            stream::iter(packets)
        );

        let data_length = i16::MAX as i32;
        let mut count = 0;
        while let Some(encoded) = encoding_stream.next().await {
            assert!(!encoded.is_empty(), "Encoded packet should not be empty");
            // 3 bit header + Binary Mask + packet type +  data_length data
            assert_eq!(encoded.len() as i32, data_length + 5);

            assert_eq!(encoded[0], 126 | BINARY_MASK);
            assert_eq!(encoded[1], ((data_length + 2) >> 8) as u8);
            assert_eq!(encoded[2], (data_length + 2) as u8);

            assert_eq!(encoded[3], BINARY_MASK);
            assert_eq!(encoded[4], PacketType::Message.as_char() as u8);
            assert_eq!(encoded[5..], (0..data_length).map(|_| 0x44).collect::<Vec<u8>>());

            count += 1;
            if count == 20 { break; }
        }
        assert_eq!(count, 20, "There should be twenty encoded packets");
    }

    // Test encoding stream with large packet sets
    #[tokio::test]
    async fn encoding_stream_functions_with_large_packet_streams() {
        let packets= (0..i16::MAX)
            .map(|_| Packet::new(
                PacketType::Message, None, Some(RawData::Text("Hello".into()))
            ))
            .collect::<Vec<Packet>>();
        let mut encoding_stream = PacketEncoderStream::new(
            stream::iter(packets)
        );

        let mut count = 0;
        while let Some(encoded) = encoding_stream.next().await {
            assert!(!encoded.is_empty(), "Encoded packet should not be empty");
            // 1 bit header + TEXT_MASK + Message Type + 5 bit "Hello"
            assert_eq!(encoded.len(), 1 + 1 + 1 + 5);
            assert_eq!(encoded[0], 7);
            assert_eq!(encoded[1], PLAIN_TEXT_MASK);
            assert_eq!(encoded[2], '4' as u8);
            assert_eq!(encoded[3..], Vec::from("Hello".as_bytes()));
            count += 1;
            if count == i16::MAX { break; }
        }
        assert_eq!(count, i16::MAX, "There should be i16::MAX encoded packets");
    }

    #[tokio::test]
    async fn test_encoding_with_async_channel() {
        let (tx, rx) = mpsc::channel(10);
        let mut encoder = PacketEncoderStream::new(tokio_stream::wrappers::ReceiverStream::new(rx));
        let sender = tx.clone();

        tokio::spawn(async move {
            for _ in 0..3 {
                let packet = Packet::new(PacketType::Message, None, Some(RawData::Text("Async".into())));
                sender.send(packet).await.unwrap();
            }
        });

        let mut count = 0;
        while let Some(encoded) = encoder.next().await {
            assert!(!encoded.is_empty(), "Encoded packet should not be empty");
            // 1 bit header + TEXT_MASK + Message Type + 5 bit "Async"
            assert_eq!(encoded.len(), 1 + 1 + 1 + 5);
            assert_eq!(encoded[0], 7);
            assert_eq!(encoded[1], PLAIN_TEXT_MASK);
            assert_eq!(encoded[2], '4' as u8);
            assert_eq!(encoded[3..], Vec::from("Async".as_bytes()));
            count += 1;
            if count >= 3 { break; } // Prevent infinite test execution
        }

        assert_eq!(count, 3, "All packets should be received and encoded");
    }
}
