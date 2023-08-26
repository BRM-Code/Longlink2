use json::{JsonValue};


pub fn process_data_packet(parsed: JsonValue){
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;
    use std::u16;
    use base64::Engine;
    use base64::engine::general_purpose;

    #[test]
    fn process_unencrypted_test() {
        let data = "dTF/GRdCvNb0wgAAEEHIMghQMgAKABQAHgABAQAA";
        let binding= general_purpose::STANDARD.decode(data).expect("Failed base64 decode");
        let data_decoded= binding.as_slice();
        let uav_id = from_utf8(&data_decoded[..2]).expect("Failed string conversion for UAV ID");
        assert_eq!("u1", uav_id); // UAV_ID
        assert_eq!(37.7749, f32::from_le_bytes(<[u8; 4]>::try_from(data_decoded[2..6].to_vec()).unwrap())); // Lat
        assert_eq!(-122.4194, f32::from_le_bytes(<[u8; 4]>::try_from(data_decoded[6..10].to_vec()).unwrap())); // Long
        assert_eq!(9.0, f32::from_le_bytes(<[u8; 4]>::try_from(data_decoded[10..14].to_vec()).unwrap())); // Vbatt
        assert_eq!(200, data_decoded[14]); // Altitude
        assert_eq!(50, data_decoded[15]); // Ground Speed
        assert_eq!(8, data_decoded[16]); // Satellites
        assert_eq!(80, data_decoded[17]); // Consumption
        assert_eq!(50, data_decoded[18]); // RSSI
        assert_eq!(10, u16::from_be_bytes(<[u8; 2]>::try_from(data_decoded[19..21].to_vec()).unwrap())); // Pitch
        assert_eq!(20, u16::from_be_bytes(<[u8; 2]>::try_from(data_decoded[21..23].to_vec()).unwrap())); // Roll
        assert_eq!(30, u16::from_be_bytes(<[u8; 2]>::try_from(data_decoded[23..25].to_vec()).unwrap())); // Heading
        assert_eq!(1, data_decoded[26]); // Arm
        assert_eq!(1, data_decoded[27]); // Sat fix
    }
}
