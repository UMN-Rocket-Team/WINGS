use chrono::NaiveDate;

use crate::models::packet::{Packet, PacketFieldValue};

/// Finding the GPS Packet withing the Byte stream and returning it
/// 
/// Because the data is mostly given in ascii, it is converted to a string, 
/// we then find the GPS packet by using the alignment character and ParseType field
/// The data is cut down to just the first line with a gps packet 
///     ( we assume one gps packet per transmission since gps packets are only sent once every couple of seconds so multiple would be highly unlikely)
/// once the data is cut down it is then sent to the parser to be 
pub fn packet_from_byte_stream(buffer: [u8;4096], gps_packet_id:usize) -> anyhow::Result<Packet>{
    let res = String::from_utf8_lossy(&buffer);
    let res2 = res.trim_matches(char::from(0));
    let packet_loc =  res2.find("@ GPS_STAT").ok_or(anyhow::anyhow!("no Gps Packet"))?;
    let mut after_at = res2.to_owned().split_off(packet_loc);
    _ = after_at.split_off(after_at.find("\r\n").expect(""));
    let arr = parser(&mut after_at)?;
    Ok(Packet { structure_id: gps_packet_id, field_data: arr })
}

/// Parses String into Vector of packet field values
/// 
/// Strings are split by spaces and then fields of interest are parsed individually
/// Time data given by the packet is simplified into a millisecond timestamp
fn parser(raw_data: &mut str) -> anyhow::Result<Vec<PacketFieldValue>>{
    let message: Vec<&str> = raw_data.split(" ").collect();
    let time_vec: Vec<&str> = message[6].split(&[':', '.']).collect();
    let mut return_vec = vec![];
    println!("{:?}",message);
    let dt = NaiveDate::from_ymd_opt(
        message[3].parse::<i32>().unwrap_or(0), 
        message[4].parse::<u32>().unwrap_or(0), 
        message[5].parse::<u32>().unwrap_or(0)).unwrap_or(NaiveDate::from_ymd_opt(2015,1,1).expect("Pre-Written Date"))
        .and_hms_milli_opt(
            time_vec[0].parse::<u32>().unwrap_or(0),
            time_vec[1].parse::<u32>().unwrap_or(0),
            time_vec[2].parse::<u32>().unwrap_or(0),
            time_vec[3].parse::<u32>().unwrap_or(0)).ok_or(anyhow::anyhow!("bad time"))?;
    return_vec.push(PacketFieldValue::Number(dt.and_utc().timestamp_millis() as f64));
    return_vec.push(PacketFieldValue::Number(message[11].parse::<f64>().unwrap_or(0.0)));
    return_vec.push(PacketFieldValue::Number(message[13].parse::<f64>().unwrap_or(0.0)));
    return_vec.push(PacketFieldValue::Number(message[15].parse::<f64>().unwrap_or(0.0)));
    return_vec.push(PacketFieldValue::Number(message[17].parse::<f64>().unwrap_or(0.0)));
    return_vec.push(PacketFieldValue::Number(message[18].parse::<f64>().unwrap_or(0.0)));
    return_vec.push(PacketFieldValue::Number(message[19].parse::<f64>().unwrap_or(0.0)));
    Ok(return_vec)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_string_output() {
        let result = parser(&mut "@ GPS_STAT 208 0000 00 00 02:53:51.907 CRC_ERR TRK $p���i��:57 Alt 4403468 lt -03.10000 ln +00.00000 Vel +16384 +16512 +0004 Fix 0 # 12 140 192  5 000_00_00 000_00_00 000_00_00 000_00_00 000_00_00 CRC: E579".to_owned());
        assert_eq!(result.unwrap(), [
            PacketFieldValue::Number(1420080831907.0), 
            PacketFieldValue::Number(4403468.0), 
            PacketFieldValue::Number(-3.1), 
            PacketFieldValue::Number(0.0), 
            PacketFieldValue::Number(16384.0), 
            PacketFieldValue::Number(16512.0), 
            PacketFieldValue::Number(4.0)]);

        
    }
}