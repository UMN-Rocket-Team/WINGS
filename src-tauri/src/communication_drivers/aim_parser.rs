use std::cmp::max;

use crate::{
    models::{self, packet::{Packet, PacketFieldValue}},
    packet_structure_manager::PacketStructureManager,
};
#[derive(Default)]
pub struct AimParser {
}

/// responsible converting raw data to packets
impl AimParser {

    /// processes the raw data queue, returning a Vector(aka. array) of the processed packets
    pub fn parse_transmission(
        &mut self,
        transmission: Vec<u8>,
        print_flag: bool,
    ) -> Vec<Packet> {
        let length = transmission[1];
        let rssi = i16::from_be_bytes(transmission[2..4].try_into().expect("Given slice has incorrect length!"));
        let snr = transmission[4];
        let mut i:usize = 3;
        let mut packets: Vec<Packet> = vec![];

        packets.push(Packet{
            structure_id: todo!(),
            field_data: vec![],
            field_meta_data: todo!(),
        });

        while i < length.into(){
            i+=2;
            let delimiter = transmission[i +1];
            let time = transmission[i];
            let mut data: Vec<f64> = vec![];
            match delimiter {
                0x02 =>{
                    let value :f64 = i16::from_be_bytes(transmission[i+2..i+4].try_into().expect("Given slice has incorrect length!")).into();
                    data.push(value/256.0);
                    i+=2;
                },
                0x03 =>{
                    let mut raw:u32 = 0;
                    raw |= (transmission[i+2] as u32) << 16;
                    raw |= (transmission[i+3] as u32) << 8;
                    raw |= transmission[i+4] as u32;
                    data.push(raw.into());
                    i+=3;
                },
                0x04 | 0x05 =>{
                    let adc:f64 = u16::from_be_bytes(transmission[i+2..i+4].try_into().expect("Given slice has incorrect length!")).into();
                    data.push((3.3 * adc) / 2.0_f64.powf(16.0));
                    i+=2;
                },
                0x06 =>{
                    let adc:f64 = u16::from_be_bytes(transmission[i+2..i+4].try_into().expect("Given slice has incorrect length!")).into();
                    data.push(adc / 100.0);
                    i+=2;
                },
                0x07 | 0x08 | 0x09 | 0x0A =>{
                    //TODO, right now this just gets ADC, still need to grab the other bytes
                    let raw =u16::from_be_bytes(transmission[i+2..i+4].try_into().expect("Given slice has incorrect length!"));
                    
                    let adc:f64 = (raw | 0b0011111111111111).into();
                    
                    data.push(5.0*adc / 16384.0);
                    i+=2;
                },
                0x0B =>{
                    //TODO, other values
                    let value :f64 = i16::from_be_bytes(transmission[i+2..i+4].try_into().expect("Given slice has incorrect length!")).into();
                    let divisor: f64 = (u8::max_value()).into();
                    data.push(value/(divisor + 1.0));
                    i+=4;
                },
                0x0C =>{
                    //TODO, other values
                    let adc:f64 = (u16::from_be_bytes(transmission[i+2..i+4].try_into().expect("Given slice has incorrect length!"))).into();
                    data.push(adc / 16.0);
                    i+=6;
                },
                0x0D =>{
                    //TODO, other values
                    data.push((u16::from_be_bytes(transmission[i+2..i+4].try_into().expect("Given slice has incorrect length!"))).into());
                    i+=6;
                },
                0x0E =>{
                    //TODO, other values
                    data.push((i32::from_be_bytes(transmission[i+2..i+6].try_into().expect("Given slice has incorrect length!"))).into());
                    i+=13;
                },
                0x0F =>{
                    data.push(i16::from_be_bytes(transmission[i+2..i+4].try_into().expect("Given slice has incorrect length!")).into());
                    i+=2;
                },
                0x10 =>{
                    //TODO, process this data
                    data.push(i16::from_be_bytes(transmission[i+2..i+4].try_into().expect("Given slice has incorrect length!")).into());
                    i+=2;
                },
                0x11 =>{
                    //TODO, process this data
                    data.push(u32::from_be_bytes(transmission[i+2..i+6].try_into().expect("Given slice has incorrect length!")).into());
                    i+=6;
                },
                0x12 =>{
                    //TODO, process this data
                    data.push(u32::from_be_bytes(transmission[i+2..i+6].try_into().expect("Given slice has incorrect length!")) as f64);
                    i+=7;
                },
                0x14 =>{
                    //TODO, process this data
                    data.push(u32::from_be_bytes(transmission[i+2..i+6].try_into().expect("Given slice has incorrect length!")) as f64);
                    i+=4;
                },
                0x15 =>{
                    //TODO, process this data
                    data.push(i16::from_be_bytes(transmission[i+2..i+4].try_into().expect("Given slice has incorrect length!")).into());
                    i+=8;
                },
                _ => {
                    i = 65;
                }
            }
            packets.push(Packet{
                structure_id: todo!(),
                field_data: todo!(),
                field_meta_data: todo!(),
            });
        }
        return packets   
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write, path::Path};

    use crate::models::packet::Packet;

    use super::{AimParser};
    //  lets the unit tests use everything in this file
    /// test for basic packet recognition and parsing
    #[test]
    fn test_parse_from_json(){
        let path = Path::new(".\\test utilities\\out.txt");
        let path2: &Path = Path::new(".\\test utilities\\timestep_02.csv");
        let mut file = std::fs::OpenOptions::new().append(true).create(true).open(path2).expect("failed to open file");
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
                    let mut aim = AimParser::default();
                    let ans = aim.parse_transmission(byte_array,false);
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
                        if (bruh_list.contains(&str.as_str())) {
                            parse_json(collector,last_str,val,indent+1,path.to_owned() + "/o:" + &str);
                            i+=1;
                        }
                    }
                }
            },
        }

    }
}

