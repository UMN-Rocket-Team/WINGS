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
        packet_structure_manager: &PacketStructureManager,
        transmission: [u8;65],
        print_flag: bool,
    ) -> Vec<Packet> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};
    //  lets the unit tests use everything in this file
    /// test for basic packet recognition and parsing
    #[test]
    fn test_parse_from_json(){
        let path = Path::new(".\\test utilities\\raw_data.json");
        println!("\n\n\nPATH: {:#?}\n\n\n", fs::canonicalize(&path));
        let json: serde_json::Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).expect("JSON was not well-formatted");
        parse_json(&json,0, "".to_owned());
        print!("\n");
    }

    fn parse_json(value: &serde_json::Value,indent:u8, path : String){
        let bruh_list = ["_index","_type"];
        match value{
            serde_json::Value::Null => {
            },
            serde_json::Value::Bool(bool) => {
                print!("\n{}/b:{}",path,bool)
            },
            serde_json::Value::Number(number) => {
                print!("\n{}/n:{}",path,number)
            },
            serde_json::Value::String(str) => {
                print!("\n{}/s:{}",path,str)
            },
            serde_json::Value::Array(values) => {
                let mut j = 0;
                for i in values{
                    if j < 10{
                        parse_json(i,indent+1, path.to_owned() + "/a");
                        j +=1;
                    }
                }
            },
            serde_json::Value::Object(map) => {
                let mut i = 0;
                for (str,val ) in map {
                    if i < 10 {
                        if !(bruh_list.contains(&str.as_str())) {
                            parse_json(val,indent+1,path.to_owned() + "/o:" + &str);
                            i+=1;
                        }
                    }
                }
            },
        }

    }
}

