use crate::{packet::*, constants::*};

#[cfg(test)]
mod single_packet_encoding_tests {
    use super::*;

    #[test]
    fn encode_binary_packet_with_binary_support() {
        let packet = Packet::new(
            PacketType::Message,
            Some(PacketOptions { compress: false }),
            Some(RawData::Binary(vec![1, 2, 3, 4, 5])),
        );

        let encoded = packet.encode(true); // supports_binary = true
        match encoded {
            RawData::Binary(data) => assert_eq!(data, vec![1, 2, 3, 4, 5]),
            _ => panic!("Expected Binary data"),
        }
    }

    #[test]
    fn encode_binary_packet_without_binary_support() {
        let packet = Packet::new(
            PacketType::Message,
            Some(PacketOptions { compress: false }),
            Some(RawData::Binary(vec![1, 2, 3, 4, 5])),
        );

        let encoded = packet.encode(false); // supports_binary = false
        match encoded {
            RawData::Text(text) => {
                assert!(text.starts_with("b")); // base64 encoded data should start with 'b'
                assert!(text.len() > 1); // Base64 encoded string should be longer than 1 char
            }
            _ => panic!("Expected Text data"),
        }
    }

    #[test]
    fn encode_text_data_with_binary_support() {
        let packet = Packet::new(
            PacketType::Message,
            Some(PacketOptions { compress: false }),
            Some(RawData::Text("Hello, world!".to_string())),
        );

        let encoded = packet.encode(true); // supports_binary = true
        match encoded {
            RawData::Text(text) => {
                assert_eq!(text, "4Hello, world!"); // Should prefix with '4' (Message type)
            }
            _ => panic!("Expected Text data"),
        }
    }

    #[test]
    fn encode_text_data_without_binary_support() {
        let packet = Packet::new(
            PacketType::Message,
            Some(PacketOptions { compress: false }),
            Some(RawData::Text("Hello, world!".to_string())),
        );

        let encoded = packet.encode(false); // supports_binary = true
        match encoded {
            RawData::Text(text) => {
                assert_eq!(text, "4Hello, world!"); // Should prefix with '4' (Message type)
            }
            _ => panic!("Expected Text data"),
        }
    }

    #[test]
    fn encode_empty_data() {
        let packet = Packet::new(
            PacketType::Ping,
            None,
            None,
        );

        let encoded = packet.encode(true); // supports_binary = true
        match encoded {
            RawData::Text(text) => {
                assert_eq!(text, "2"); // Should just be the character '2' for Ping type
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
        let packet = Packet::new(
            PacketType::Message,
            Some(PacketOptions { compress: false }),
            Some(RawData::Binary(vec![1, 2, 3, 4, 5])),
        );

        let encoded = packet.encode_binary(); // supports_binary = true
        assert_eq!(encoded, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn force_binary_encode_text_data() {
        let packet = Packet::new(
            PacketType::Message,
            Some(PacketOptions { compress: false }),
            Some(RawData::Text("Hello world!".into())),
        );

        let encoded = packet.encode_binary(); // supports_binary = true // supports_binary = true
        assert_eq!(encoded, String::from("4Hello world!").into_bytes());
    }

    #[test]
    fn force_binary_encode_no_data() {
        let packet = Packet::new(
            PacketType::Open,
            Some(PacketOptions { compress: false }),
            None,
        );

        let encoded = packet.encode_binary(); // supports_binary = true // supports_binary = true
        assert_eq!(encoded, String::from("0").into_bytes());
    }
}

#[cfg(test)]
mod payload_encoding_tests{
    use super::*;

    #[test]
    fn encode_payload_packets_with_no_data() {
        let packet1 = Packet::new(
            PacketType::Ping,
            None,
            None,
        );
        let packets = (0..20).map(|_| packet1.clone()).collect::<Vec<Packet>>();
        let encoded_payload = Packet::encode_payload(&packets, true);

        // Should be a
        let mut expected_payload: BinaryType = Vec::new();
        for _ in 0..20{
            expected_payload.push(PLAIN_TEXT_MASK);
            expected_payload.extend_from_slice(String::from("2").as_bytes());
            expected_payload.push(SEPARATOR_BYTE);
        }
        expected_payload.pop();
        assert_eq!(encoded_payload, expected_payload);
    }


    #[test]
    fn encode_payload_packets_with_mixed_data() {
        let packet1 = Packet::new(
            PacketType::Message,
            Some(PacketOptions { compress: false }),
            Some(RawData::Binary(vec![1, 2, 3])),
        );
        let packet2 = Packet::new(
            PacketType::Ping,
            None,
            Some(RawData::Text("Ping".to_string())),
        );

        let mut packets = vec![];
        for _ in 0..20 { packets.extend_from_slice(&vec![packet1.clone(), packet2.clone()]) }

        let encoded_payload = Packet::encode_payload(&packets, true);

        let mut expected_payload: BinaryType = vec![];
        for _ in 0..20{
            // Packet 1
            expected_payload.push(BINARY_MASK);
            expected_payload.extend_from_slice(&vec![1,2,3]);
            expected_payload.push(SEPARATOR_BYTE);
            // Packet 2
            expected_payload.push(PLAIN_TEXT_MASK);
            expected_payload.extend_from_slice(String::from("2Ping").as_bytes());
            expected_payload.push(SEPARATOR_BYTE);
        }
        expected_payload.pop();

        assert_eq!(encoded_payload, expected_payload);
    }

    #[test]
    fn encode_payload_packets_with_large_data() {
        let data = (0..i16::MAX).map(|_| 0x44).collect::<Vec<u8>>();
        let packet1 = Packet::new(
            PacketType::Ping,
            None,
            Some(RawData::Binary(data)),
        );

        let packets = (0..20).map(|_| packet1.clone()).collect::<Vec<Packet>>();
        let encoded_payload = Packet::encode_payload(&packets, true);

        let mut expected_payload: BinaryType = vec![];
        for _ in 0..20{
            expected_payload.push(BINARY_MASK);
            expected_payload.extend_from_slice(&(0..i16::MAX).map(|_| 0x44).collect::<Vec<u8>>());
            expected_payload.push(SEPARATOR_BYTE);
        }
        expected_payload.pop();
        assert_eq!(encoded_payload, expected_payload);
    }

    #[test]
    fn encode_large_payload() {
        let data = (0..20).map(|_| 0x44).collect::<Vec<u8>>();
        let packet1 = Packet::new(
            PacketType::Ping,
            None,
            Some(RawData::Binary(data)),
        );

        let packets = (0..i16::MAX).map(|_| packet1.clone()).collect::<Vec<Packet>>();
        let encoded_payload = Packet::encode_payload(&packets, true);

        let mut expected_payload: BinaryType = vec![];
        for _ in 0..i16::MAX{
            expected_payload.push(BINARY_MASK);
            expected_payload.extend_from_slice(&(0..20).map(|_| 0x44).collect::<Vec<u8>>());
            expected_payload.push(SEPARATOR_BYTE);
        }
        expected_payload.pop();
        assert_eq!(encoded_payload, expected_payload);
    }



    #[test]
    fn encode_empty_payload() {
        let packets: Vec<Packet> = Vec::new();
        let encoded_payload = Packet::encode_payload(&packets, true);

        assert!(encoded_payload.is_empty()); // Should result in an empty payload
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
            assert_eq!(encoded[0], 6, "First byte should be 6 -> (Category byte + 5 bytes for Hello");
            assert_eq!(&encoded[1..], "4Hello".as_bytes(), "Last bytes should be \"Hello\" as bytes");
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
            assert_eq!(encoded[0], 1, "First byte should be 1");
            assert_eq!(encoded[1], 50, "Second byte should be '2' as a byte");
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
                Some(RawData::Binary((0..i16::MAX).map(|_| 4).collect::<Vec<u8>>()))
            ))
            .collect::<Vec<Packet>>();
        let mut encoding_stream = PacketEncoderStream::new(
            stream::iter(packets)
        );

        let mut count = 0;
        while let Some(encoded) = encoding_stream.next().await {
            assert!(!encoded.is_empty(), "Encoded packet should not be empty");
            assert_eq!(encoded.len() as i32, i16::MAX as i32 + 3);
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
            count += 1;
            if count >= 3 { break; } // Prevent infinite test execution
        }

        assert_eq!(count, 3, "All packets should be received and encoded");
    }
}
