use crate::{RawData, packet::*};

#[cfg(test)]
mod packet_creation_tests {
    use super::*;

    #[test]
    fn full_packet_creation() {
        let packet_type = PacketType::Message;
        let options = Some(PacketOptions { compress: true });
        let data = Some(RawData::Binary(vec![1, 2, 3, 4, 5]));

        let packet = Packet::new(packet_type.clone(), options.clone(), data.clone());

        assert_eq!(packet._type, packet_type);
        assert_eq!(packet.options, options);
        assert_eq!(packet.data, data);
    }

    #[test]
    fn packet_creation_without_options() {
        let packet_type = PacketType::Message;
        let data = Some(RawData::Text(String::from("Hello World!")));

        let packet = Packet::new(packet_type.clone(), None, data.clone());

        assert_eq!(packet._type, packet_type);
        assert!(packet.options.is_none());
        assert_eq!(packet.data, data);
    }

    #[test]
    fn packet_creation_with_empty_data() {
        let packet = Packet::new(PacketType::Message, None, Some(RawData::Binary(Vec::new())));

        assert_eq!(packet._type, PacketType::Message);
        assert_eq!(packet.data, Some(RawData::Binary(Vec::new())));
    }

    #[test]
    fn packet_creation_with_large_data() {
        let large_data = RawData::Binary(vec![0; 10_000]);
        let packet = Packet::new(PacketType::Message, None, Some(large_data));

        if let Some(RawData::Binary(bin)) = packet.data {
            assert_eq!(bin.len(), 10_000);
        } else { assert!(false) }
    }

    #[test]
    fn packet_creation_without_options_or_data() {
        let packet_type = PacketType::Ping;
        let packet = Packet::new(packet_type, None, None);

        assert_eq!(packet._type, PacketType::Ping);
        assert!(packet.options.is_none());
        assert!(packet.data.is_none());
    }

    #[test]
    fn error_packet_creation() {
        let message = "An error occurred";
        let packet = Packet::error(message);

        assert_eq!(packet._type, PacketType::Error);
        assert!(packet.options.is_none());
        assert_eq!(packet.data, Some(RawData::Binary(message.bytes().collect())));
    }
}


#[cfg(test)]
mod packet_type_transcribing_tests {
    use super::*;

    #[test]
    fn packet_type_from_str() {
        assert_eq!(PacketType::from_str("open"), Some(PacketType::Open));
        assert_eq!(PacketType::from_str("close"), Some(PacketType::Close));
        assert_eq!(PacketType::from_str("ping"), Some(PacketType::Ping));
        assert_eq!(PacketType::from_str("pong"), Some(PacketType::Pong));
        assert_eq!(PacketType::from_str("message"), Some(PacketType::Message));
        assert_eq!(PacketType::from_str("upgrade"), Some(PacketType::Upgrade));
        assert_eq!(PacketType::from_str("noop"), Some(PacketType::Noop));
        assert_eq!(PacketType::from_str("error"), Some(PacketType::Error));
        assert_eq!(PacketType::from_str("invalid"), None);
    }

    #[test]
    fn packet_type_from_char() {
        assert_eq!(PacketType::from_char('0'), Some(PacketType::Open));
        assert_eq!(PacketType::from_char('1'), Some(PacketType::Close));
        assert_eq!(PacketType::from_char('2'), Some(PacketType::Ping));
        assert_eq!(PacketType::from_char('3'), Some(PacketType::Pong));
        assert_eq!(PacketType::from_char('4'), Some(PacketType::Message));
        assert_eq!(PacketType::from_char('5'), Some(PacketType::Upgrade));
        assert_eq!(PacketType::from_char('6'), Some(PacketType::Noop));
        assert_eq!(PacketType::from_char('e'), Some(PacketType::Error));
        assert_eq!(PacketType::from_char('x'), None);
    }

    #[test]
    fn packet_type_as_str() {
        assert_eq!(PacketType::Open.as_str(), "open");
        assert_eq!(PacketType::Close.as_str(), "close");
        assert_eq!(PacketType::Ping.as_str(), "ping");
        assert_eq!(PacketType::Pong.as_str(), "pong");
        assert_eq!(PacketType::Message.as_str(), "message");
        assert_eq!(PacketType::Upgrade.as_str(), "upgrade");
        assert_eq!(PacketType::Noop.as_str(), "noop");
        assert_eq!(PacketType::Error.as_str(), "error");
    }

    #[test]
    fn packet_type_as_char() {
        assert_eq!(PacketType::Open.as_char(), '0');
        assert_eq!(PacketType::Close.as_char(), '1');
        assert_eq!(PacketType::Ping.as_char(), '2');
        assert_eq!(PacketType::Pong.as_char(), '3');
        assert_eq!(PacketType::Message.as_char(), '4');
        assert_eq!(PacketType::Upgrade.as_char(), '5');
        assert_eq!(PacketType::Noop.as_char(), '6');
        assert_eq!(PacketType::Error.as_char(), 'e');
    }
}

#[cfg(test)]
mod packet_clone_and_equality_tests {
    use super::*;

    #[test]
    fn packet_clone() {
        let packet = Packet::new(
            PacketType::Message,
            Some(PacketOptions { compress: true }),
            Some(RawData::Binary(vec![1, 2, 3])),
        );

        let cloned_packet = packet.clone();

        assert_eq!(packet, cloned_packet);
    }

    #[test]
    fn packet_partial_eq() {
        let mut _type = PacketType::Message;
        let mut options = Some(PacketOptions { compress: true });
        let mut data = Some(RawData::Text(String::from("Hello World")));
        build_compare_packets(_type.clone(), options.clone(), data.clone(), true);
        build_compare_packets(_type.clone(), options.clone(), data.clone(), false);

        data = Some(RawData::Binary(Vec::new()));
        build_compare_packets(_type.clone(), options.clone(), data.clone(), true);
        build_compare_packets(_type.clone(), options.clone(), data.clone(), false);

        data = Some(RawData::Binary(Vec::from(vec![0x01, 0x02, 0x03])));
        build_compare_packets(_type.clone(), options.clone(), data.clone(), true);
        build_compare_packets(_type.clone(), options.clone(), data.clone(), false);

        _type = PacketType::Ping;
        options = None;
        data = None;
        build_compare_packets(_type.clone(), options.clone(), data.clone(), true);
        build_compare_packets(_type.clone(), options.clone(), data.clone(), false);


        _type = PacketType::Open;
        build_compare_packets(_type.clone(), options.clone(), data.clone(), true);
        build_compare_packets(_type.clone(), options.clone(), data.clone(), false);
    }

    fn build_compare_packets(
        _type: PacketType,
        packet_options: Option<PacketOptions>,
        data: Option<RawData>,
        equals: bool
    ){
        let diff_type = PacketType::Message;
        let diff_opt = Some(PacketOptions { compress: true });
        let diff_data = Some(RawData::Binary(Vec::from(vec![0x80])));

        let compare_packet = Packet::new(
            if equals { _type.clone() } else { diff_type },
            if equals { packet_options.clone() } else { diff_opt },
            if equals { data.clone() } else { diff_data },
        );
        let packet = Packet::new(
            _type,
            packet_options,
            data,
        );

        if equals {
            assert_eq!(packet, compare_packet);
        } else {
            assert_ne!(packet, compare_packet);
        }
    }
}

