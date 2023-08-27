use std::str::from_utf8;
use base64::Engine;
use base64::engine::general_purpose;

struct TelemetryData {
    latitude: f32,
    longitude: f32,
    vbatt: f32,
    altitude: u8,
    ground_speed: u8,
    satellites: u8,
    consumption: u8,
    rssi: u8,
    pitch: u16,
    roll: u16,
    heading: u16,
    arm: bool,
    sat_fix: bool,
}

// Takes Base64 encoded data packet and converts it to TelemetryData and extracts UAV_ID
pub fn process_data_packet(parsed: &str) -> (&str, TelemetryData) {
    let binding= general_purpose::STANDARD.decode(parsed).expect("Failed base64 decode");
    let data_decoded= binding.as_slice();
    let uav_id = from_utf8(&data_decoded[..2]).expect("Failed string conversion for UAV ID");
    let telemetry = TelemetryData{
        latitude: f32::from_le_bytes(<[u8; 4]>::try_from(data_decoded[2..6].to_vec()).expect("Lat unpack failed")),
        longitude: f32::from_le_bytes(<[u8; 4]>::try_from(data_decoded[6..10].to_vec()).expect("Long unpack failed")),
        vbatt: f32::from_le_bytes(<[u8; 4]>::try_from(data_decoded[10..14].to_vec()).expect("Vbatt unpack failed")),
        altitude: data_decoded[14],
        ground_speed: data_decoded[15],
        satellites: data_decoded[16],
        consumption: data_decoded[17],
        rssi: data_decoded[18],
        pitch: u16::from_be_bytes(<[u8; 2]>::try_from(data_decoded[19..21].to_vec()).expect("Pitch unpack failed")),
        roll: u16::from_be_bytes(<[u8; 2]>::try_from(data_decoded[21..23].to_vec()).expect("Roll unpack failed")),
        heading: u16::from_be_bytes(<[u8; 2]>::try_from(data_decoded[23..25].to_vec()).expect("Heading unpack failed")),
        arm: data_decoded[26] != 0,
        sat_fix: data_decoded[27] != 0,
    };
    (uav_id, telemetry)
}

#[cfg(test)]
mod tests {
    use crate::packet::process_data_packet;

    #[test]
    fn process_unencrypted_test() {
        let data = "dTF/GRdCvNb0wgAAEEHIMghQMgAKABQAHgABAQAA";
        let (uav_id, telemetry) = process_data_packet(data);

        assert_eq!("u1", uav_id); // UAV_ID
        assert_eq!(37.7749, telemetry.latitude); // Lat
        assert_eq!(-122.4194, telemetry.longitude); // Long
        assert_eq!(9.0, telemetry.vbatt); // Vbatt
        assert_eq!(200, telemetry.altitude); // Altitude
        assert_eq!(50, telemetry.ground_speed); // Ground Speed
        assert_eq!(8, telemetry.satellites); // Satellites
        assert_eq!(80, telemetry.consumption); // Consumption
        assert_eq!(50, telemetry.rssi); // RSSI
        assert_eq!(10, telemetry.pitch); // Pitch
        assert_eq!(20, telemetry.roll); // Roll
        assert_eq!(30, telemetry.heading); // Heading
        assert_eq!(true , telemetry.arm); // Arm
        assert_eq!(true, telemetry.sat_fix); // Sat fix
    }
}
