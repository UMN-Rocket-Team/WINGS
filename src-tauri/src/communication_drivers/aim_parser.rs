use std::time::SystemTime;

use crate::{
    models::{
        packet::{Packet, PacketFieldValue},
        packet_parser::PacketParser,
        packet_structure::PacketFieldType,
    },
    packet_structure_manager::PacketStructureManager,
};

const AIM_FRAME_LEN: usize = 64;

//hardcoded packet names that are used by this parser
const META: &str = "Aim_Meta";
const ACCEL_Z: &str = "Aim_AccelZ";
const PRESSURE: &str = "Aim_Pressure";
const COMP_BATT: &str = "Aim_BatComp";
const EJECT_BATT: &str = "Aim_BatEject";
const TEMP: &str = "Aim_Temp";
const LINE_A: &str = "Aim_LineA";
const LINE_B: &str = "Aim_LineB";
const LINE_C: &str = "Aim_LineC";
const LINE_D: &str = "Aim_LineD";
const ACCEL_XY: &str = "Aim_AccelXY";
const GYRO: &str = "Aim_GyroXYZ";
const MAG: &str = "Aim_MagXYZ";
const GPS: &str = "Aim_GPSLLSOL";
const RSSI: &str = "Aim_RSSI";
const STATUS: &str = "Aim_Status";
const IDENTIFIER: &str = "Aim_Ident";
const GPS_TIME: &str = "Aim_GPSTime";
const TIMESTAMP: &str = "Aim_TimeStamp";
const ORIENTATION: &str = "Aim_Orientation";
pub struct AimParser {
    unparsed_data: Vec<u8>,
    init_time: SystemTime,
}

//holds all packetIdsRelatedToThisParser,Fields are filled in at initialization
struct PacketIdList {
    meta: usize,
    accel_z: usize,
    pressure: usize,
    comp_batt: usize,
    eject_batt: usize,
    temp: usize,
    line_a: usize,
    line_b: usize,
    line_c: usize,
    line_d: usize,
    accel_xy: usize,
    gyro: usize,
    mag: usize,
    gps: usize,
    rssi: usize,
    status: usize,
    identifier: usize,
    gps_time: usize,
    timestamp: usize,
    orientation: usize,
}

impl Default for AimParser {
    fn default() -> Self {
        Self {
            unparsed_data: Vec::new(),
            init_time: SystemTime::now(),
        }
    }
}

fn packet_id_by_name(
    packet_structure_manager: &PacketStructureManager,
    name: &str,
) -> anyhow::Result<usize> {
    packet_structure_manager
        .packet_structures
        .iter()
        .find(|packet_structure| packet_structure.name == name)
        .map(|packet_structure| packet_structure.id)
        .ok_or_else(|| anyhow::anyhow!("Aim packet structure not registered: {name}"))
}

fn get_packet_ids(
    packet_structure_manager: &PacketStructureManager,
) -> anyhow::Result<PacketIdList> {
    Ok(PacketIdList {
        meta: packet_id_by_name(packet_structure_manager, META)?,
        accel_z: packet_id_by_name(packet_structure_manager, ACCEL_Z)?,
        pressure: packet_id_by_name(packet_structure_manager, PRESSURE)?,
        comp_batt: packet_id_by_name(packet_structure_manager, COMP_BATT)?,
        eject_batt: packet_id_by_name(packet_structure_manager, EJECT_BATT)?,
        temp: packet_id_by_name(packet_structure_manager, TEMP)?,
        line_a: packet_id_by_name(packet_structure_manager, LINE_A)?,
        line_b: packet_id_by_name(packet_structure_manager, LINE_B)?,
        line_c: packet_id_by_name(packet_structure_manager, LINE_C)?,
        line_d: packet_id_by_name(packet_structure_manager, LINE_D)?,
        accel_xy: packet_id_by_name(packet_structure_manager, ACCEL_XY)?,
        gyro: packet_id_by_name(packet_structure_manager, GYRO)?,
        mag: packet_id_by_name(packet_structure_manager, MAG)?,
        gps: packet_id_by_name(packet_structure_manager, GPS)?,
        rssi: packet_id_by_name(packet_structure_manager, RSSI)?,
        status: packet_id_by_name(packet_structure_manager, STATUS)?,
        identifier: packet_id_by_name(packet_structure_manager, IDENTIFIER)?,
        gps_time: packet_id_by_name(packet_structure_manager, GPS_TIME)?,
        timestamp: packet_id_by_name(packet_structure_manager, TIMESTAMP)?,
        orientation: packet_id_by_name(packet_structure_manager, ORIENTATION)?,
    })
}

/// responsible converting raw data to packets
impl PacketParser for AimParser {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn register_packet_structures(
        packet_structure_manager: &mut PacketStructureManager,
    ) -> anyhow::Result<()> {
        packet_structure_manager.enforce_packet_fields(META, vec!["System time", "RSSI", "SNR"]);
        packet_structure_manager
            .enforce_packet_fields(ACCEL_Z, vec!["System time", "Delta time", "Z acceleration"]);
        packet_structure_manager
            .enforce_packet_fields(PRESSURE, vec!["System time", "Delta time", "Pressure(Pa)"]);
        packet_structure_manager
            .enforce_packet_fields(COMP_BATT, vec!["System time", "Delta time", "ADC(V)"]);
        packet_structure_manager
            .enforce_packet_fields(EJECT_BATT, vec!["System time", "Delta time", "ADC(V)"]);
        packet_structure_manager
            .enforce_packet_fields(TEMP, vec!["System time", "Delta time", "Temperature"]);
        packet_structure_manager.enforce_packet_fields(
            LINE_A,
            vec!["System time", "Delta time", "ADC", "Is_On", "Is_Input"],
        );
        packet_structure_manager.enforce_packet_fields(
            LINE_B,
            vec!["System time", "Delta time", "ADC", "Is_On", "Is_Input"],
        );
        packet_structure_manager.enforce_packet_fields(
            LINE_C,
            vec!["System time", "Delta time", "ADC", "Is_On", "Is_Input"],
        );
        packet_structure_manager.enforce_packet_fields(
            LINE_D,
            vec!["System time", "Delta time", "ADC", "Is_On", "Is_Input"],
        );
        packet_structure_manager.enforce_packet_fields(
            ACCEL_XY,
            vec![
                "System time",
                "Delta time",
                "X acceleration",
                "Y acceleration",
            ],
        );
        packet_structure_manager.enforce_packet_fields(
            GYRO,
            vec![
                "System time",
                "Delta time",
                "X rotation",
                "Y rotation",
                "Z rotation",
            ],
        );
        packet_structure_manager.enforce_packet_fields(
            MAG,
            vec!["System time", "Delta time", "X flux", "Y flux", "Z flux"],
        );
        packet_structure_manager.enforce_packet_fields(
            GPS,
            vec![
                "System time",
                "Delta time",
                "Lat",
                "Long",
                "MSL(mm)",
                "lock",
                "sat_num",
            ],
        );
        packet_structure_manager
            .enforce_packet_fields(RSSI, vec!["System time", "Delta time", "RSSI"]);
        packet_structure_manager.enforce_packet_fields(
            STATUS,
            vec![
                "System time",
                "Delta time",
                "State",
                "Line D on",
                "Line C on",
                "Line B on",
                "Line A on",
                "Line A continuity",
                "Line B continuity",
                "Line C continuity",
                "Line D continuity",
                "Line A input",
                "Line B input",
                "Line C input",
                "Line D input",
            ],
        );
        packet_structure_manager
            .enforce_packet_fields(IDENTIFIER, vec!["System time", "Delta time", "Identifier"]);
        packet_structure_manager.enforce_packet_fields(
            GPS_TIME,
            vec![
                "System time",
                "Delta time",
                "iTOW",
                "GPS Week",
                "Valid time",
                "Valid leap secs",
                "leap secs",
            ],
        );
        packet_structure_manager
            .enforce_packet_fields(TIMESTAMP, vec!["System time", "Delta time", "Timestamp"]);
        packet_structure_manager.enforce_packet_fields(
            ORIENTATION,
            vec![
                "System time",
                "Delta time",
                "Quat x",
                "Quat y",
                "Quat z",
                "Quat w",
            ],
        );

        Ok(())
    }

    fn get_unparsed_data(&mut self) -> &mut Vec<u8> {
        self.unparsed_data.as_mut()
    }

    fn parse_packets(
        &mut self,
        packet_structure_manager: &PacketStructureManager,
        print_flag: bool,
    ) -> anyhow::Result<Vec<Packet>> {
        if print_flag {
            println!("Unparsed data length: {}", self.unparsed_data.len());
        }

        let mut packets: Vec<Packet> = vec![];
        let packet_ids = get_packet_ids(packet_structure_manager)?;

        let mut parsed_bytes = 0;

        while self.unparsed_data.len().saturating_sub(parsed_bytes) >= AIM_FRAME_LEN {
            let transmission = &self.unparsed_data[parsed_bytes..(parsed_bytes + AIM_FRAME_LEN)];
            if transmission.len() <= 63 {
                break;
            }

            let time_received = self.init_time.elapsed()?.as_millis() as f64;
            let length = transmission[1];
            let rssi = i16::from_be_bytes(
                transmission[2..4]
                    .try_into()
                    .expect("Given slice has incorrect length!"),
            ) as f64;
            let snr = transmission[4] as f64;
            let mut i: usize = 3;

            packets.push(Packet::default(
                packet_ids.meta,
                vec![
                    PacketFieldValue::Number(time_received),
                    PacketFieldValue::Number(rssi),
                    PacketFieldValue::Number(snr),
                ],
            ));

            while i < length.into() {
                i += 2;
                let delimiter = transmission[i + 1];
                let delta_time = transmission[i];
                let type_id: usize;
                let mut data: Vec<PacketFieldValue> = vec![
                    PacketFieldValue::Number(time_received),
                    PacketFieldValue::Number(time_received + (delta_time as f64)),
                ];

                match delimiter {
                    0x02 => {
                        type_id = packet_ids.accel_z;

                        let mut value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 2..i + 4])?;
                        value.edit_number(&mut |x| *x / 256.0);

                        data.push(value);

                        i += 2;
                    }
                    0x03 => {
                        type_id = packet_ids.pressure;

                        let value =
                            PacketFieldType::UnsignedTwoFour.parse(&transmission[i + 2..i + 5])?;

                        data.push(value);

                        i += 3;
                    }
                    0x04 | 0x05 => {
                        if delimiter == 0x04 {
                            type_id = packet_ids.comp_batt;
                        } else {
                            type_id = packet_ids.eject_batt;
                        }

                        let mut value =
                            PacketFieldType::UnsignedShort.parse(&transmission[i + 2..i + 4])?;
                        value.edit_number(&mut |x| (3.3 * *x) / 2.0_f64.powf(16.0));

                        data.push(value);

                        i += 2;
                    }
                    0x06 => {
                        type_id = packet_ids.temp;

                        let mut value =
                            PacketFieldType::UnsignedShort.parse(&transmission[i + 2..i + 4])?;
                        value.edit_number(&mut |x| *x / 100.0);

                        data.push(value);

                        i += 2;
                    }
                    // ..= is Searching through the range of values 0x07,0x08,0x09,and 0x0A
                    0x07..=0x0A => {
                        if delimiter == 0x07 {
                            type_id = packet_ids.line_a;
                        } else if delimiter == 0x08 {
                            type_id = packet_ids.line_b;
                        } else if delimiter == 0x09 {
                            type_id = packet_ids.line_c;
                        } else {
                            type_id = packet_ids.line_d;
                        }

                        let mut value =
                            PacketFieldType::UnsignedShort.parse(&transmission[i + 2..i + 4])?;
                        value.edit_number(&mut |x| ((*x as u16) | 0b0011111111111111) as f64);
                        value.edit_number(&mut |x| 5.0 * (*x) / 16384.0);

                        let is_on =
                            PacketFieldType::Bool.parse(&[transmission[i + 2] & 0b10000000])?;
                        let is_input =
                            PacketFieldType::Bool.parse(&[transmission[i + 2] & 0b01000000])?;

                        data.push(value);
                        data.push(is_on);
                        data.push(is_input);

                        i += 2;
                    }
                    0x0B => {
                        type_id = packet_ids.accel_xy;

                        let mut x_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 2..i + 4])?;
                        x_value.edit_number(&mut |x| *x / 256.0);
                        let mut y_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 4..i + 6])?;
                        y_value.edit_number(&mut |x| *x / 256.0);

                        data.push(x_value);
                        data.push(y_value);

                        i += 4;
                    }
                    0x0C => {
                        type_id = packet_ids.gyro;

                        let mut x_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 2..i + 4])?;
                        x_value.edit_number(&mut |x| *x / 70.0);
                        let mut y_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 4..i + 6])?;
                        y_value.edit_number(&mut |x| *x / 70.0);
                        let mut z_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 6..i + 8])?;
                        z_value.edit_number(&mut |x| *x / 70.0);

                        data.push(x_value);
                        data.push(y_value);
                        data.push(z_value);

                        i += 6;
                    }
                    0x0D => {
                        type_id = packet_ids.mag;

                        let x_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 2..i + 4])?;
                        let y_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 4..i + 6])?;
                        let z_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 6..i + 8])?;

                        data.push(x_value);
                        data.push(y_value);
                        data.push(z_value);

                        i += 6;
                    }
                    0x0E => {
                        type_id = packet_ids.gps;
                        let lat =
                            PacketFieldType::SignedInteger.parse(&transmission[i + 2..i + 6])?;
                        let long =
                            PacketFieldType::SignedInteger.parse(&transmission[i + 6..i + 10])?;
                        let msl =
                            PacketFieldType::SignedInteger.parse(&transmission[i + 10..i + 14])?;
                        let lock =
                            PacketFieldType::Bool.parse(&[transmission[i + 14] & 0b00100000])?;
                        let sat_num = PacketFieldType::UnsignedByte
                            .parse(&[transmission[i + 14] & 0b00011111])?;

                        data.push(lat);
                        data.push(long);
                        data.push(msl);
                        data.push(lock);
                        data.push(sat_num);

                        i += 13;
                    }
                    0x0F => {
                        type_id = packet_ids.rssi;

                        let value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 2..i + 4])?;
                        data.push(value);

                        i += 2;
                    }
                    0x10 => {
                        type_id = packet_ids.status;

                        data.push(
                            PacketFieldType::UnsignedByte
                                .parse(&[(transmission[i + 2] & 0b11110000) >> 4])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 2] & 0b00001000])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 2] & 0b00000100])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 2] & 0b00000010])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 2] & 0b00000001])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 3] & 0b10000000])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 3] & 0b01000000])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 3] & 0b00100000])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 3] & 0b00010000])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 3] & 0b00001000])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 3] & 0b00000100])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 3] & 0b00000010])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 3] & 0b00000001])?,
                        );

                        i += 2;
                    }
                    0x11 => {
                        type_id = packet_ids.identifier;

                        data.push(PacketFieldType::String.parse(&transmission[i + 2..i + 8])?);

                        i += 6;
                    }
                    0x12 => {
                        type_id = packet_ids.gps_time;

                        data.push(
                            PacketFieldType::UnsignedInteger.parse(&transmission[i + 2..i + 6])?,
                        );
                        data.push(
                            PacketFieldType::UnsignedShort.parse(&transmission[i + 6..i + 8])?,
                        );

                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 8] & 0b10000000])?,
                        );
                        data.push(
                            PacketFieldType::Bool.parse(&[transmission[i + 8] & 0b01000000])?,
                        );
                        data.push(
                            PacketFieldType::UnsignedByte
                                .parse(&[transmission[i + 8] & 0b00111111])?,
                        );

                        i += 7;
                    }
                    0x14 => {
                        type_id = packet_ids.timestamp;

                        let value =
                            PacketFieldType::UnsignedInteger.parse(&transmission[i + 2..i + 4])?;
                        data.push(value);

                        i += 4;
                    }
                    0x15 => {
                        type_id = packet_ids.orientation;

                        let x_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 2..i + 4])?;
                        let y_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 4..i + 6])?;
                        let z_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 6..i + 8])?;
                        let w_value =
                            PacketFieldType::SignedShort.parse(&transmission[i + 8..i + 10])?;

                        data.push(x_value);
                        data.push(y_value);
                        data.push(z_value);
                        data.push(w_value);

                        i += 8;
                    }
                    _ => {
                        return Err(anyhow::anyhow!("found unknown packet!"));
                    }
                }
                packets.push(Packet {
                    structure_id: type_id,
                    field_data: data,
                });
            }

            parsed_bytes += AIM_FRAME_LEN;
        }

        self.unparsed_data.drain(0..parsed_bytes);
        Ok(packets)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::{
        models::{packet::Packet, packet_parser::PacketParser},
        packet_structure_manager::PacketStructureManager,
    };

    use super::AimParser;

    /// user validated test for basic packet recognition and parsing
    #[test]
    fn test_parse_from_json() {
        let path = Path::new("test_utilities")
            .join("aim_test_files")
            .join("out.txt");
        let json: serde_json::Value = serde_json::from_str(&fs::read_to_string(path).unwrap())
            .expect("JSON was not well-formatted");
        let mut json_string_array = vec![];
        if let serde_json::Value::Array(values) = json {
            json_string_array = values;
        }

        let mut collector: Vec<Vec<Packet>> = vec![];

        for value in json_string_array {
            if let serde_json::Value::String(str) = value {
                let string_by_bytes = str.split(":");
                let mut byte_array: Vec<u8> = vec![];
                for string in string_by_bytes {
                    byte_array.append(&mut hex::decode(string).expect("uh oh stinky"));
                }
                let mut packet_structure_manager = PacketStructureManager::default();
                AimParser::register_packet_structures(&mut packet_structure_manager)
                    .expect("packet structure registration");

                let mut aim = AimParser::default();
                aim.push_data(&byte_array, false);
                let ans = aim
                    .parse_packets(&packet_structure_manager, false)
                    .expect("parser");
                collector.push(ans);
            }
        }
    }
}
