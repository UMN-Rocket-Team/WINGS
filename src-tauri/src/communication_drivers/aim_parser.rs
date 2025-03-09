use std::{sync::Arc, time::SystemTime};

use crate::{models::{packet::{Packet, PacketFieldValue}, packet_structure::PacketFieldType}, packet_structure_manager::PacketStructureManager};

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
    init_time: SystemTime,
    packet_ids: PacketIdList,
}

//holds all packetIdsRelatedToThisParser,Fields are filled in at initialization
pub struct PacketIdList {
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

/// responsible converting raw data to packets
impl AimParser {

    pub fn default(ps_manager: &mut PacketStructureManager) -> AimParser{
        return AimParser{
            init_time: SystemTime::now(),
            packet_ids: PacketIdList{
                meta: ps_manager.get_packet_structure_by_name(META),
                accel_z: ps_manager.get_packet_structure_by_name(ACCEL_Z),
                pressure: ps_manager.get_packet_structure_by_name(PRESSURE),
                comp_batt: ps_manager.get_packet_structure_by_name(COMP_BATT),
                eject_batt: ps_manager.get_packet_structure_by_name(EJECT_BATT),
                temp: ps_manager.get_packet_structure_by_name(TEMP),
                line_a: ps_manager.get_packet_structure_by_name(LINE_A),
                line_b: ps_manager.get_packet_structure_by_name(LINE_B),
                line_c: ps_manager.get_packet_structure_by_name(LINE_C),
                line_d: ps_manager.get_packet_structure_by_name(LINE_D),
                accel_xy: ps_manager.get_packet_structure_by_name(ACCEL_XY),
                gyro: ps_manager.get_packet_structure_by_name(GYRO),
                mag: ps_manager.get_packet_structure_by_name(MAG),
                gps: ps_manager.get_packet_structure_by_name(GPS),
                rssi: ps_manager.get_packet_structure_by_name(RSSI),
                status: ps_manager.get_packet_structure_by_name(STATUS),
                identifier: ps_manager.get_packet_structure_by_name(IDENTIFIER),
                gps_time: ps_manager.get_packet_structure_by_name(GPS_TIME),
                timestamp: ps_manager.get_packet_structure_by_name(TIMESTAMP),
                orientation: ps_manager.get_packet_structure_by_name(ORIENTATION)
            },
            
        }
    }
    /// processes the raw data queue, returning a Vector(aka. array) of the processed packets
    pub fn parse_transmission(
        &mut self,
        transmission: &mut Vec<u8>,
        packets:&mut Vec<Packet>
    ) -> anyhow::Result<()> {
        let time_received = self.init_time.elapsed()?.as_millis() as f64;
        let length = transmission[1];
        let rssi = i16::from_be_bytes(transmission[2..4].try_into().expect("Given slice has incorrect length!")) as f64;
        let snr = transmission[4] as f64;
        let mut i: usize = 3;

        packets.push(Packet::default(
            self.packet_ids.meta,
            vec![
                PacketFieldValue::Number(time_received),
                PacketFieldValue::Number(rssi),
                PacketFieldValue::Number(snr),
            ]
        ));

        while i < length.into(){
            i+=2;
            let delimiter = transmission[i +1];
            let delta_time = transmission[i];
            let type_id: usize;
            let mut data: Vec<PacketFieldValue> = vec![PacketFieldValue::Number(time_received),PacketFieldValue::Number(delta_time.into())];

            match delimiter {
                0x02 =>{
                    type_id = self.packet_ids.accel_z;

                    let mut value = PacketFieldType::SignedShort.parse(&transmission[i+2..i+4])?;
                    value.edit_number(&mut |x| *x/256.0);

                    data.push(value);

                    i+=2;
                },
                0x03 =>{
                    type_id = self.packet_ids.pressure;

                    let value = PacketFieldType::UnsignedTwoFour.parse(&transmission[i+2..i+5])?;

                    data.push(value);

                    i+=3;
                    
                },
                0x04 | 0x05 =>{
                    if delimiter == 0x04{
                        type_id = self.packet_ids.comp_batt;
                    }
                    else{
                        type_id = self.packet_ids.eject_batt;
                    }

                    let mut value = PacketFieldType::UnsignedShort.parse(&transmission[i+2..i+4])?;
                    value.edit_number(&mut |x|{(3.3 * *x) / 2.0_f64.powf(16.0)});

                    data.push(value);

                    i+=2;
                },
                0x06 =>{
                    type_id = self.packet_ids.temp;

                    let mut value = PacketFieldType::UnsignedShort.parse(&transmission[i+2..i+4])?;
                    value.edit_number(&mut |x| *x/100.0);

                    data.push(value);

                    i+=2;
                },
                0x07 | 0x08 | 0x09 | 0x0A =>{
                    if delimiter == 0x07{
                        type_id = self.packet_ids.line_a;
                    }
                    else if delimiter == 0x08{
                        type_id = self.packet_ids.line_b;
                    }
                    else if delimiter == 0x09{
                        type_id = self.packet_ids.line_c;
                    }
                    else{
                        type_id = self.packet_ids.line_d;
                    }

                    let mut value = PacketFieldType::UnsignedShort.parse(&transmission[i+2..i+4])?;
                    value.edit_number(&mut |x| ((*x as u16) | 0b0011111111111111) as f64);
                    value.edit_number(&mut |x| 5.0*(*x) / 16384.0);

                    let is_on = PacketFieldType::Bool.parse(&[transmission[i+2] & 0b10000000])?;
                    let is_input = PacketFieldType::Bool.parse(&[transmission[i+2] & 0b01000000])?;

                    data.push(value);
                    data.push(is_on);
                    data.push(is_input);

                    i+=2;
                },
                0x0B =>{
                    type_id = self.packet_ids.accel_xy;

                    let mut x_value = PacketFieldType::SignedShort.parse(&transmission[i+2..i+4])?;
                    x_value.edit_number(&mut |x| *x/256.0);
                    let mut y_value = PacketFieldType::SignedShort.parse(&transmission[i+4..i+6])?;
                    y_value.edit_number(&mut |x| *x/256.0);
                    
                    data.push(x_value);
                    data.push(y_value);

                    i+=4;
                },
                0x0C =>{
                    type_id = self.packet_ids.gyro;

                    let mut x_value = PacketFieldType::SignedShort.parse(&transmission[i+2..i+4])?;
                    x_value.edit_number(&mut |x| *x/70.0);
                    let mut y_value = PacketFieldType::SignedShort.parse(&transmission[i+4..i+6])?;
                    y_value.edit_number(&mut |x| *x/70.0);
                    let mut z_value = PacketFieldType::SignedShort.parse(&transmission[i+6..i+8])?;
                    z_value.edit_number(&mut |x| *x/70.0);
                    
                    data.push(x_value);
                    data.push(y_value);
                    data.push(z_value);

                    i+=6;
                },
                0x0D =>{
                    type_id = self.packet_ids.mag;

                    let x_value = PacketFieldType::SignedShort.parse(&transmission[i+2..i+4])?;
                    let y_value = PacketFieldType::SignedShort.parse(&transmission[i+4..i+6])?;
                    let z_value = PacketFieldType::SignedShort.parse(&transmission[i+6..i+8])?;

                    data.push(x_value);
                    data.push(y_value);
                    data.push(z_value);

                    i+=6;
                },
                0x0E =>{
                    type_id = self.packet_ids.gps;
                    let lat = PacketFieldType::SignedInteger.parse(&transmission[i+2..i+6])?;
                    let long = PacketFieldType::SignedInteger.parse(&transmission[i+6..i+10])?;
                    let msl = PacketFieldType::SignedInteger.parse(&transmission[i+10..i+14])?;
                    let lock = PacketFieldType::Bool.parse(&[transmission[i+14] & 0b00100000])?;
                    let sat_num = PacketFieldType::UnsignedByte.parse(&[transmission[i+14] & 0b00011111])?;

                    data.push(lat);
                    data.push(long);
                    data.push(msl);
                    data.push(lock);
                    data.push(sat_num);

                    i+=13;
                },
                0x0F =>{
                    type_id = self.packet_ids.rssi;

                    let value = PacketFieldType::SignedShort.parse(&transmission[i+2..i+4])?;
                    data.push(value);

                    i+=2;
                },
                0x10 =>{
                    type_id = self.packet_ids.status;

                    data.push(PacketFieldType::UnsignedByte.parse(&[(transmission[i+2] & 0b11110000) >> 4])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+2] & 0b00001000])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+2] & 0b00000100])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+2] & 0b00000010])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+2] & 0b00000001])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+3] & 0b10000000])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+3] & 0b01000000])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+3] & 0b00100000])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+3] & 0b00010000])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+3] & 0b00001000])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+3] & 0b00000100])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+3] & 0b00000010])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+3] & 0b00000001])?);

                    i+=2;
                },
                0x11 =>{
                    type_id = self.packet_ids.identifier;

                    data.push(PacketFieldType::String.parse(&transmission[i+2..i+8])?);

                    i+=6;
                },
                0x12 =>{
                    type_id = self.packet_ids.gps_time;

                    data.push(PacketFieldType::UnsignedInteger.parse(&transmission[i+2..i+6])?);
                    data.push(PacketFieldType::UnsignedShort.parse(&transmission[i+6..i+8])?);

                    data.push(PacketFieldType::Bool.parse(&[transmission[i+8] & 0b10000000])?);
                    data.push(PacketFieldType::Bool.parse(&[transmission[i+8] & 0b01000000])?);
                    data.push(PacketFieldType::UnsignedByte.parse(&[transmission[i+8] & 0b00111111])?);


                    i+=7;
                },
                0x14 =>{
                    type_id = self.packet_ids.timestamp;

                    let value = PacketFieldType::UnsignedInteger.parse(&transmission[i+2..i+4])?;
                    data.push(value);

                    i+=4;
                },
                0x15 =>{
                    type_id = self.packet_ids.orientation;

                    let x_value = PacketFieldType::SignedShort.parse(&transmission[i+2..i+4])?;
                    let y_value = PacketFieldType::SignedShort.parse(&transmission[i+4..i+6])?;
                    let z_value = PacketFieldType::SignedShort.parse(&transmission[i+6..i+8])?;
                    let w_value = PacketFieldType::SignedShort.parse(&transmission[i+8..i+10])?;

                    data.push(x_value);
                    data.push(y_value);
                    data.push(z_value);
                    data.push(w_value);

                    i+=8;
                },
                _ => {
                    return Err(anyhow::anyhow!("found unknown packet!"));
                }
            }
            packets.push(Packet{
                structure_id: type_id,
                field_data: data,
            });
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::{models::packet::Packet, packet_structure_manager::PacketStructureManager};

    use super::AimParser;
    //  lets the unit tests use everything in this file
    /// test for basic packet recognition and parsing
    #[test]
    fn test_parse_from_json(){
        let path = Path::new(".\\test utilities\\out.txt");
        //let path2: &Path = Path::new(".\\test utilities\\timestep_02.csv");
        //let mut file = std::fs::OpenOptions::new().append(true).create(true).open(path2).expect("failed to open file");
        let json: serde_json::Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).expect("JSON was not well-formatted");
        let mut json_string_array = vec![];
        match json {
            serde_json::Value::Array(values) => {
                json_string_array = values;
            },
            _ => {

            }
        }

        let mut collector: Vec<Vec<Packet>> = vec![];

        for value in json_string_array {
            match value {
                serde_json::Value::String(str) => {
                    let string_by_bytes = str.split(":");
                    let mut byte_array: Vec<u8> = vec![];
                    for string in string_by_bytes{
                        byte_array.append(&mut hex::decode(string).expect("uh oh stinky"));
                    }
                    let mut aim = AimParser::default(&mut PacketStructureManager::default());
                    let mut ans = vec![];
                    aim.parse_transmission(&mut byte_array,&mut ans).expect("parser");
                    collector.push(ans);
               },
                _ => {
    
                }
            }
        }     
    }
    fn parse_json(collector: &mut Vec<String>,last_str: &mut Vec<String>,value: &serde_json::Value,indent:u8, path : String){
        let bruh_list = ["_source","layers","usbhid.data_tree","usbhid.data.array"];

        match value{
            serde_json::Value::Null => {
            },
            serde_json::Value::Bool(bool) => {
                //print!("\n{}/b:{}",path,bool)
            },
            serde_json::Value::Number(number) => {
                //print!("\n{}/n:{}",path,number)
            },
            serde_json::Value::String(str) => {
                if path.contains("usbhid.data.array"){
                    if !path.contains("usbhid.data.array.usage"){
                        if !(str.starts_with("12") || str.starts_with("13")){
                            let new_str = &str;
                            if !str.starts_with("02:00"){
                                collector.push(str.to_string());
                                println!("{}",str);
                            }
                            (*last_str).push(new_str.to_string());

                        }
                    }
                }
            },
            serde_json::Value::Array(values) => {
                let mut j = 0;
                for i in values{
                    if j > -1{
                        parse_json(collector,last_str,i,indent+1, path.to_owned() + "/a");
                        j +=1;
                    }
                }
            },
            serde_json::Value::Object(map) => {
                let mut i = 0;
                for (str,val ) in map {
                    if i > -1 {
                        if bruh_list.contains(&str.as_str()) {
                            parse_json(collector,last_str,val,indent+1,path.to_owned() + "/o:" + &str);
                            i+=1;
                        }
                    }
                }
            },
        }

    }
}

