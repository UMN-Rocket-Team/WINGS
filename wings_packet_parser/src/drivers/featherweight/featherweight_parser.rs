use chrono::NaiveDate;

use anyhow::{bail, Context};

use crate::{
    models::{
        packet::{Packet, PacketFieldValue},
        packet_parser::PacketParser,
    },
    packet_structure_manager::PacketStructureManager,
};

const FEATHERWEIGHT_GPS_NAME: &str = "FW GPS";
const GPS_MARKER: &[u8] = b"@ GPS_STAT";
const MAX_BUFFER_BYTES: usize = 1024;

#[derive(Default)]
pub struct FeatherweightParser {
    unparsed_data: Vec<u8>,
}

fn find_subslice(data: &[u8], needle: &[u8], start_index: usize) -> Option<usize> {
    if needle.is_empty() || start_index >= data.len() {
        return None;
    }

    data[start_index..]
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|index| index + start_index)
}

fn gps_packet_id(packet_structure_manager: &PacketStructureManager) -> anyhow::Result<usize> {
    packet_structure_manager
        .packet_structures
        .iter()
        .find(|packet_structure| packet_structure.name == FEATHERWEIGHT_GPS_NAME)
        .map(|packet_structure| packet_structure.id)
        .ok_or_else(|| anyhow::anyhow!("Featherweight packet structure not registered"))
}

impl PacketParser for FeatherweightParser {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn register_packet_structures(
        packet_structure_manager: &mut PacketStructureManager,
    ) -> anyhow::Result<()> {
        packet_structure_manager.enforce_packet_fields(
            FEATHERWEIGHT_GPS_NAME,
            vec![
                "TimeStamp", //Milliseconds
                "Altitude",  //Feet
                "Lat",       //Degrees
                "Long",      //Degrees
                "Vel Lat",   //Feet per second
                "Vel Long",  //Feet per second
                "Vel Vert",  //Feet per second
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

        let gps_packet_id = gps_packet_id(packet_structure_manager)?;
        let mut packets = Vec::new();
        let mut search_index = 0;
        let mut consumed_up_to = 0;

        while let Some(packet_start_index) =
            find_subslice(&self.unparsed_data, GPS_MARKER, search_index)
        {
            let maybe_line_end = self.unparsed_data[packet_start_index..]
                .iter()
                .position(|byte| *byte == b'\n');

            let Some(line_end_relative_index) = maybe_line_end else {
                break;
            };

            let line_end_index = packet_start_index + line_end_relative_index;
            let mut line_bytes = &self.unparsed_data[packet_start_index..line_end_index];
            if let Some((last_byte, rest)) = line_bytes.split_last() {
                if *last_byte == b'\r' {
                    line_bytes = rest;
                }
            }

            let line = String::from_utf8_lossy(line_bytes);
            let parsed_fields = parser(line.trim_matches(char::from(0)))?;

            packets.push(Packet {
                structure_id: gps_packet_id,
                field_data: parsed_fields,
            });

            consumed_up_to = line_end_index + 1;
            search_index = consumed_up_to;
        }

        if consumed_up_to > 0 {
            self.unparsed_data.drain(0..consumed_up_to);
        } else if self.unparsed_data.len() > MAX_BUFFER_BYTES {
            let bytes_to_drain = self.unparsed_data.len() - MAX_BUFFER_BYTES;
            self.unparsed_data.drain(0..bytes_to_drain);
        }

        Ok(packets)
    }
}

/// Parses String into Vector of packet field values
///
/// Strings are split by spaces and then fields of interest are parsed individually
/// Time data given by the packet is simplified into a millisecond timestamp
fn parser(raw_data: &str) -> anyhow::Result<Vec<PacketFieldValue>> {
    let message: Vec<&str> = raw_data.split_whitespace().collect();
    if message.len() <= 19 {
        bail!("gps packet too short ({} fields)", message.len());
    }

    let time_str = message.get(6).context("gps packet missing time field")?;
    let time_vec: Vec<&str> = time_str.split(&[':', '.']).collect();
    if time_vec.len() != 4 {
        bail!("bad time format: '{time_str}'");
    }

    let mut return_vec = vec![];

    let dt = NaiveDate::from_ymd_opt(
        message[3].parse::<i32>().unwrap_or(0),
        message[4].parse::<u32>().unwrap_or(0),
        message[5].parse::<u32>().unwrap_or(0),
    )
    .unwrap_or_else(|| NaiveDate::from_ymd_opt(2015, 1, 1).expect("Pre-Written Date"))
    .and_hms_milli_opt(
        time_vec[0].parse::<u32>().unwrap_or(0),
        time_vec[1].parse::<u32>().unwrap_or(0),
        time_vec[2].parse::<u32>().unwrap_or(0),
        time_vec[3].parse::<u32>().unwrap_or(0),
    )
    .ok_or(anyhow::anyhow!("bad time"))?;

    return_vec.push(PacketFieldValue::Number(
        dt.and_utc().timestamp_millis() as f64
    ));
    return_vec.push(PacketFieldValue::Number(
        message
            .get(11)
            .context("gps packet missing altitude")?
            .parse::<f64>()
            .unwrap_or(0.0),
    ));
    return_vec.push(PacketFieldValue::Number(
        message
            .get(13)
            .context("gps packet missing latitude")?
            .parse::<f64>()
            .unwrap_or(0.0),
    ));
    return_vec.push(PacketFieldValue::Number(
        message
            .get(15)
            .context("gps packet missing longitude")?
            .parse::<f64>()
            .unwrap_or(0.0),
    ));
    return_vec.push(PacketFieldValue::Number(
        message
            .get(17)
            .context("gps packet missing velocity x")?
            .parse::<f64>()
            .unwrap_or(0.0),
    ));
    return_vec.push(PacketFieldValue::Number(
        message
            .get(18)
            .context("gps packet missing velocity y")?
            .parse::<f64>()
            .unwrap_or(0.0),
    ));
    return_vec.push(PacketFieldValue::Number(
        message
            .get(19)
            .context("gps packet missing velocity z")?
            .parse::<f64>()
            .unwrap_or(0.0),
    ));
    Ok(return_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_string_output() {
        let result = parser("@ GPS_STAT 208 0000 00 00 02:53:51.907 CRC_ERR TRK $p���i��:57 Alt 4403468 lt -03.10000 ln +00.00000 Vel +16384 +16512 +0004 Fix 0 # 12 140 192  5 000_00_00 000_00_00 000_00_00 000_00_00 000_00_00 CRC: E579");
        assert_eq!(
            result.unwrap(),
            [
                PacketFieldValue::Number(1420080831907.0),
                PacketFieldValue::Number(4403468.0),
                PacketFieldValue::Number(-3.1),
                PacketFieldValue::Number(0.0),
                PacketFieldValue::Number(16384.0),
                PacketFieldValue::Number(16512.0),
                PacketFieldValue::Number(4.0)
            ]
        );
    }
}
