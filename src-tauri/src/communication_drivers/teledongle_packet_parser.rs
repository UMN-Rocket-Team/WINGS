use std::cmp::max;

use crate::{
    models::{self, packet::{Packet, PacketFieldValue}},
    packet_structure_manager::PacketStructureManager,
};

#[derive(Default)]
pub struct AltosPacketParser {
    unparsed_data: Vec<u8>,
    iterator: u64,
    last: u64,
}

/// responsible converting raw data to packets
impl AltosPacketParser {
    // adds new unparsed data
    pub fn push_data(&mut self, data: &[u8], print_flag: bool) {
        self.unparsed_data.extend(data);
        if print_flag {
            println!("Unparsed data: {:02X?}", self.unparsed_data);
        }
    }

    /// processes the raw data queue, returning a Vector(aka. array) of the processed packets
    pub fn parse_packets(
        &mut self,
        packet_structure_manager: &PacketStructureManager,
        print_flag: bool
    ) -> anyhow::Result<Vec<Packet>> {
        if print_flag {
            println!("Unparsed data length: {}", self.unparsed_data.len());
        }
        // println!("{:#?}",self.unparsed_data);
        let mut packets: Vec<Packet> = vec![];
        
        let mut last_successful_match_end_index: Option<usize> = None;

            for j in 0..packet_structure_manager.packet_structures.len() {
                let packet_structure = &packet_structure_manager.packet_structures[j];
              
                if print_flag {
                    println!("At index {}, matching structure {}", 0, j);
                }

                if packet_structure.delimiters.is_empty() {
                    println!("no delimiters!");
                    println!("{}",packet_structure.name);
                    continue;
                }

                if packet_structure.size() > (self.unparsed_data.len() + (packet_structure.delimiters[0].offset_in_packet)){
                    if print_flag {
                        println!("Packet out of bounds");
                    }
                    continue;
                }
                
                if !is_delimiter_match(
                    &self.unparsed_data,
                    packet_structure.delimiters[0].offset_in_packet +2,
                    &packet_structure.delimiters[0].identifier,false
                ) { 
                    if print_flag {
                        println!("- First delimiter did not match");
                    }
                    continue;
                }

                let packet_start_index =  2;

                if let Some(last_successful_match_end_index) = last_successful_match_end_index {
                    // Use < instead of <= as the "last index" points to the byte *after*
                    // the packet ended.
                    if packet_start_index < last_successful_match_end_index {
                        // The current packet cannot overlap with a previous one``
                        if print_flag {
                            println!("- Overlaps with previous packet");
                        }
                        continue;
                    }
                }

                let mut is_remaining_delimiters_matched = true;

                for delimiter in &packet_structure.delimiters[1..] {
                    let delimiter_start_index = packet_start_index + delimiter.offset_in_packet;
                    if !is_delimiter_match(
                        &self.unparsed_data,
                        delimiter_start_index,
                        &delimiter.identifier,
                        print_flag
                    ) {
                        is_remaining_delimiters_matched = false;
                        break;
                    }
                }

                if !is_remaining_delimiters_matched {
                    if print_flag{
                        println!("- Remaining delimiters did not match");
                    }
                    continue;
                }

                if !checksum(
                    &self.unparsed_data,
                    packet_start_index,
                    print_flag
                ) {
                    if print_flag {
                        println!("- CRC check failed");
                    }
                    continue;
                }   

                // The packet is a match, parse its data
                let mut field_data: Vec<PacketFieldValue> =
                vec![PacketFieldValue::Number(0.0); packet_structure.fields.len()];
                for (k,field_data_item) in field_data.iter_mut().enumerate().take(packet_structure.fields.len()) {
                    let field = &packet_structure.fields[k];
                    let field_start_index = packet_start_index + field.offset_in_packet;
                    *field_data_item = field.r#type.parse(
                        &self.unparsed_data
                            [field_start_index..(field_start_index + field.r#type.size()?)],
                    )?;
                }
                //START AltusMetrum, timestamp code
                if packet_structure.name == "telemega_kalman" || packet_structure.name == "TeleMetrum v1.x Sensor Data" || packet_structure.name == "TeleMetrum v2 Sensor Data"{
                    let mut timestamp = serde_json::from_str::<u64>(&(serde_json::to_string(&field_data[0]).unwrap_or_default())).unwrap_or_default() + self.iterator;
                    if timestamp < self.last{
                        println!("turnover");
                        self.iterator += 65535;
                        timestamp += 65535;
                    }
                    self.last = timestamp;
                    field_data[0] = models::packet::PacketFieldValue::Number(timestamp as f64);
                }
                
                //END AltusMetrum, timestamp code
                if print_flag {
                    println!("MATCHED: {:02X?}", &self.unparsed_data[packet_start_index..(packet_start_index + packet_structure.size())]);
                }
                packets.push(Packet {
                    structure_id: packet_structure.id,
                    field_data,
                });

                // This points to the index *after* the packet ends.
                last_successful_match_end_index =
                    Some(packet_start_index + packet_structure.size());
            }

        // Throw away any garbage data that remains so that it does not have to be re-parsed
        let last_parsed_index = max(
            self.unparsed_data.len().saturating_sub(packet_structure_manager.maximum_packet_structure_size),
            last_successful_match_end_index.unwrap_or(0),
        );
        if print_flag {
            println!("LPI: {}", last_parsed_index);
        }
        self.unparsed_data = vec![];
        Ok(packets)
    }
}

//checks if the delimiter of a packet can be found in the given data
fn is_delimiter_match(data: &[u8], start_index: usize, delimiter_identifier: &[u8],print_flag: bool) -> bool {
    if start_index + delimiter_identifier.len() > data.len() {
        return false;
    }

    for j in 0..delimiter_identifier.len() {
        if print_flag {
            print!("{:02X?} ",data[start_index + j]);
            println!("{:02X?}",delimiter_identifier[j]);
        }
        if data[start_index + j] != delimiter_identifier[j] {
            return false;
        }
    }
    true
}

fn checksum(data: &[u8], start_index: usize, print_flag: bool) -> bool {
    if start_index == 0 {
        return false;
    }

    let len: usize = data[start_index-1] as usize; // Length of each AltusMetrum packet stored at byte before start index
    if start_index + len >= data.len() {
        return false;
    }

    if print_flag {
        println!("Second to last byte in packet: {:02X?}", data[start_index+len-1]);
    }

    // Checking if 7th bit in second to last byte in packet is set. Then we know CRC is correct according to AltuMetrum docs.
    data[start_index+len-1] & 0x80 > 0 
}

#[cfg(test)]
mod parser_tests {
    use models::packet_structure::PacketCRC;

    use crate::models::packet_structure::PacketStructure;

    use super::*;//lets the unit tests use everything in this file
    /// test for basic packet recognition and parsing
    #[test]
    fn test_basic_parsing(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure::make_default("Test Structure".to_owned());
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5],true);
        let mut packet_parser = AltosPacketParser::default();
        let data = [0x11,0xBA,0x5E,0xBA,
                    0x10,0x00,
                    0x08,0x00,
                    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                    0x01,0x00,
                    0x02,0x00,
                    0x03,
                    0x04,
                    0x00,0x00,
                    0x00,0x00,0x00,0x00,
                    0x1E,0xAB,0x11,0xCA,
                    0xF2 // crc
                ]; 
        
        let crc = PacketCRC {
            length: 1,
            offset_in_packet: data.len()-1
        };

        p_structure.packet_crc.push(crc);
        let id = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();


        packet_parser.push_data(&data,false);
        let parsed = packet_parser.parse_packets(&packet_structure_manager,true).expect("parser failed");
        assert_eq!(parsed[0].structure_id,id);//does the packet have the right ID?
        assert_eq!(parsed[0].field_data[0],PacketFieldValue::Number(0.0));//does the data parse correctly?
        assert_eq!(parsed[0].field_data[1],PacketFieldValue::Number(1.0));
        assert_eq!(parsed[0].field_data[2],PacketFieldValue::Number(2.0));
        assert_eq!(parsed[0].field_data[3],PacketFieldValue::Number(3.0));
        assert_eq!(parsed[0].field_data[4],PacketFieldValue::Number(4.0));
    }
    
    /// test that data isn't mistaken for packets
    #[test]
    fn can_data_be_mistaken_for_delimiters(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure::make_default("Test Structure".to_owned());
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5],true);
        packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut packet_parser = AltosPacketParser::default();
        let data = [0x11,0xBA,0x5E,0xBA,
                    0x10,0x00,
                    0x08,0x00,
                    0x11,0xBA,0x5E,0xBA,
                    0x10,0x00,
                    0x08,0x00,
                    0x01,0x00,
                    0x02,0x00,
                    0x03,
                    0x04,
                    0x00,0x00,
                    0x00,0x00,0x00,0x00,
                    0x1E,0xAB,0x11,0xCA,
                    0x1E,0xAB,0x11,0xCA];
        packet_parser.push_data(&data, false);
        let parsed = packet_parser.parse_packets(&packet_structure_manager,false).expect("parser failed");
        assert_eq!(parsed.len(),1);//is only the first packet parsed?
        assert_eq!(parsed[0].field_data[1],PacketFieldValue::Number(1.0));//does some of the data still get parsed correctly?
        assert_eq!(parsed[0].field_data[2],PacketFieldValue::Number(2.0));
        assert_eq!(parsed[0].field_data[3],PacketFieldValue::Number(3.0));
        assert_eq!(parsed[0].field_data[4],PacketFieldValue::Number(4.0));
    }

    /// test for packets of slightly longer or shorter length than expected
    #[test]
    fn bad_data_test(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure::make_default("Test Structure".to_owned());
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5],true);
        packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut packet_parser = AltosPacketParser::default();
        let data = [0x11,0xBA,0x5E,0xBA,
                    0x10,0x00,
                    0x08,0x00,
                    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                    0x01,0x00,
                    0x02,0x00,
                    0x03,
                    0x04,
                    0x00,0x00,0x00,//one too long
                    0x00,0x00,0x00,0x00,
                    0x1E,0xAB,0x11,0xCA,
                    0x11,0xBA,0x5E,0xBA,
                    0x10,0x00,
                    0x08,0x00,
                    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                    0x01,0x00,
                    0x02,0x00,
                    0x03,
                    0x04,
                    0x00,//one too short
                    0x00,0x00,0x00,0x00,
                    0x1E,0xAB,0x11,0xCA];
        packet_parser.push_data(&data,false);
        let parsed = packet_parser.parse_packets(&packet_structure_manager,false).expect("parsing error");
        assert_eq!(parsed.len(),0);//did we accidentally parse any packets?
    }
    
    /// test consecutive packets
    #[test]
    fn consecutive_parsing_test(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure::make_default("Test Structure".to_owned());
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5],true);
        packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut packet_parser = AltosPacketParser::default();
        let data = [0x11,0xBA,0x5E,0xBA,
                    0x10,0x00,
                    0x08,0x00,
                    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                    0x01,0x00,
                    0x02,0x00,
                    0x03,
                    0x04,
                    0x00,0x00,
                    0x00,0x00,0x00,0x00,
                    0x1E,0xAB,0x11,0xCA,
                    0xBA,0xBB,0xE1,];//garbage data
        packet_parser.push_data(&data,false);
        packet_parser.push_data(&data,false);
        packet_parser.push_data(&data,false);//push data 3 times
        let data = [0x11,0xBA,0x5E,0xBA,
                    0x10,0x00,
                    0x08,0x00,
                    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                    0x05,0x00,
                    0x06,0x00,
                    0x07,
                    0x08,//changing variables
                    0x00,0x00,
                    0x00,0x00,0x00,0x00,
                    0x1E,0xAB,0x11,0xCA,
                    0xBA,0xBB,0xE1,];//garbage data
        packet_parser.push_data(&data,false);
        packet_parser.push_data(&data,false);//push data 2 more times
        let parsed = packet_parser.parse_packets(&packet_structure_manager,false).expect("parser failed");
        assert_eq!(parsed.len(),5);//did we catch all the packets?
        assert_eq!(parsed[2].field_data[1],PacketFieldValue::Number(1.0));//did we parse the first group of packets correctly
        assert_eq!(parsed[2].field_data[2],PacketFieldValue::Number(2.0));
        assert_eq!(parsed[2].field_data[3],PacketFieldValue::Number(3.0));
        assert_eq!(parsed[2].field_data[4],PacketFieldValue::Number(4.0));
        assert_eq!(parsed[4].field_data[1],PacketFieldValue::Number(5.0));//did we parse the second group of packets correctly
        assert_eq!(parsed[4].field_data[2],PacketFieldValue::Number(6.0));
        assert_eq!(parsed[4].field_data[3],PacketFieldValue::Number(7.0));
        assert_eq!(parsed[4].field_data[4],PacketFieldValue::Number(8.0));
    }
    
    /// test parsing with multiple packet structures, make sure to look at ID's
    #[test]
    fn multiple_structures(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure::make_default("Test Structure".to_owned());
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5],true);
        let id1 = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut wacky_structure = PacketStructure::make_default("Wacky Structure".to_owned());
        wacky_structure.ez_make("i16 fa1a1a1a u8 u64", &["";3],true);
        let id2 = packet_structure_manager.register_packet_structure(&mut wacky_structure).unwrap();
        let mut packet_parser = AltosPacketParser::default();
        let data = [0x11,0xBA,0x5E,0xBA,
                    0x10,0x00,
                    0x08,0x00,
                    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                    0x01,0x00,
                    0x02,0x00,
                    0x03,
                    0x04,
                    0x00,0x00,
                    0x00,0x00,0x00,0x00,
                    0x1E,0xAB,0x11,0xCA,
                    0xBA,0xBB,0xE1];
        packet_parser.push_data(&data,false);
        let data2 = [0x01,0x00,
                    0x00,0x00,
                    0x1A,0x1A,0x1A,0xFA,
                    0x02,
                    0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                    0x03,0x00,0x00,0x00,0x00,0x00,0x00,0x00,];
        packet_parser.push_data(&data2,true);
        let parsed = packet_parser.parse_packets(&packet_structure_manager,true).expect("parser failed");
        assert_eq!(parsed.len(),2); // are packets parsed?
        assert_eq!(parsed[0].structure_id,id1);//does the packet have the right ID?
        assert_eq!(parsed[0].field_data[0],PacketFieldValue::Number(0.0));//does the data parse correctly?
        assert_eq!(parsed[0].field_data[1],PacketFieldValue::Number(1.0));
        assert_eq!(parsed[0].field_data[2],PacketFieldValue::Number(2.0));
        assert_eq!(parsed[0].field_data[3],PacketFieldValue::Number(3.0));
        assert_eq!(parsed[0].field_data[4],PacketFieldValue::Number(4.0));
        assert_eq!(parsed[1].structure_id,id2);//does the packet have the right ID?
        assert_eq!(parsed[1].field_data[0],PacketFieldValue::Number(1.0));//does the data parse correctly?
        assert_eq!(parsed[1].field_data[1],PacketFieldValue::Number(2.0));
        assert_eq!(parsed[1].field_data[2],PacketFieldValue::Number(3.0));
    }

    // test parsing with multiple ps's that have the same first delimiter
    #[test]
    fn same_first_delim(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure::make_default("Test Structure".to_owned());
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5],true);
        let id1 = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut wacky_structure = PacketStructure::make_default("Test Structure Variation".to_owned());
        wacky_structure.ez_make("ba5eba11 0020 0008 i64 u32 i8 _4 deadbeef", &["";3],true);
        let id2 = packet_structure_manager.register_packet_structure(&mut wacky_structure).unwrap();
        let mut packet_parser = AltosPacketParser::default();
        let data = [0x11,0xBA,0x5E,0xBA,
                    0x20,0x00,
                    0x08,0x00,
                    0x05,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                    0x00,0x00,0x01,0x00,
                    0x03,
                    0x00,0x00,0x00,
                    0x00,0x00,0x00,0x00,
                    0xEF,0xBE,0xAD,0xDE];
        packet_parser.push_data(&data,false);
        let data2 = [0x11,0xBA,0x5E,0xBA,
                    0x10,0x00,
                    0x08,0x00,
                    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                    0x01,0x00,
                    0x02,0x00,
                    0x03,
                    0x04,
                    0x00,0x00,
                    0x00,0x00,0x00,0x00,
                    0x1E,0xAB,0x11,0xCA,
                    0xBA,0xBB,0xE1];
        packet_parser.push_data(&data2,true);
        let parsed = packet_parser.parse_packets(&packet_structure_manager,true).expect("parser failed");
        assert_eq!(parsed.len(),2); // are packets parsed?
        assert_eq!(parsed[0].structure_id,id2);//does the packet have the right ID?
        assert_eq!(parsed[0].field_data[0],PacketFieldValue::Number(5.0));//does the data parse correctly?
        assert_eq!(parsed[0].field_data[1],PacketFieldValue::Number(0x10000 as f64));
        assert_eq!(parsed[0].field_data[2],PacketFieldValue::Number(3.0));
        assert_eq!(parsed[1].structure_id,id1);//does the packet have the right ID?
        assert_eq!(parsed[1].field_data[0],PacketFieldValue::Number(0.0));//does the data parse correctly?
        assert_eq!(parsed[1].field_data[1],PacketFieldValue::Number(1.0));
        assert_eq!(parsed[1].field_data[2],PacketFieldValue::Number(2.0));
        assert_eq!(parsed[1].field_data[3],PacketFieldValue::Number(3.0));
        assert_eq!(parsed[1].field_data[4],PacketFieldValue::Number(4.0));
    }

    // test for when packets dont make it into the pushed data state
    #[test]
    fn delimiter_led_packet_half_in_buffer(){
        let packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure::make_default("Test Structure".to_owned());
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8", &["";5],true);
        let mut packet_parser = AltosPacketParser::default();
        let data = [0x11,0xBA,0x5E,0xBA,
                    0x10,0x00,
                    0x08,0x00,
                    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                    0x01,0x00,
                    0x02,0x00,
                    0x03];
        packet_parser.push_data(&data,false);
        let parsed = packet_parser.parse_packets(&packet_structure_manager,false).expect("parser failed");
        assert_eq!(parsed,vec![]);//did we not parse anything?
    }

}
