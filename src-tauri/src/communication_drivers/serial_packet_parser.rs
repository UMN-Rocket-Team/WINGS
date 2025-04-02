use std::cmp::max;

use crate::{
    models::{self, packet::{Packet, PacketFieldValue}},
    packet_structure_manager::PacketStructureManager,
};

#[derive(Default)]
pub struct SerialPacketParser {
    unparsed_data: Vec<u8>,
    iterator: u64,
    last: u64,
}

/// responsible converting raw data to packets
impl SerialPacketParser {
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
    ) -> Vec<Packet> {
        if print_flag {
            println!("Unparsed data length: {}", self.unparsed_data.len());
        }
        let mut packets: Vec<Packet> = vec![];

        let mut last_successful_match_end_index: Option<usize> = None;

        let mut maximum_index = self.unparsed_data.len();
        maximum_index = maximum_index.checked_sub(packet_structure_manager.minimum_packet_structure_size).unwrap_or(maximum_index); //don't look for packets that cant be completely inside the buffer
        maximum_index += packet_structure_manager.maximum_first_delimiter + 1; //look at the furthest point where a first delimiter could appear
        for i in 0..maximum_index {
            // Try to find a matching packet for the data
            for j in 0..packet_structure_manager.packet_structures.len() {
                let packet_structure = &packet_structure_manager.packet_structures[j];
                if print_flag {
                    println!("At index {}, matching structure {}", i, j);
                }
                if i + packet_structure.size() > (self.unparsed_data.len() + (packet_structure.delimiters[0].offset_in_packet)){
                    if print_flag {
                        println!("Packet out of bounds");
                    }
                    continue;
                }
                if !is_delimiter_match(
                    &self.unparsed_data,
                    i,
                    &packet_structure.delimiters[0].identifier,print_flag
                ) { 
                    if print_flag {
                        println!("- First delimiter did not match");
                    }
                    continue;
                }

                if packet_structure.delimiters[0].offset_in_packet > i {
                    println!("- Packet starts before data begins!");
                    continue;
                }

                let packet_start_index = i - packet_structure.delimiters[0].offset_in_packet;

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

                // The packet is a match, parse its data
                let mut field_data: Vec<PacketFieldValue> =
                    vec![PacketFieldValue::UnsignedByte(0); packet_structure.fields.len()];
                for k in 0..packet_structure.fields.len() {
                    let field = &packet_structure.fields[k];
                    let field_start_index = packet_start_index + field.offset_in_packet;
                    field_data[k] = field.r#type.parse(
                        &self.unparsed_data
                            [field_start_index..(field_start_index + field.r#type.size())],
                    );
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
                    field_data[0] = models::packet::PacketFieldValue::UnsignedLong(timestamp);
                }
                
                //END AltusMetrum, timestamp code
                if print_flag {
                    println!("MATCHED: {:02X?}", &self.unparsed_data[packet_start_index..(packet_start_index + packet_structure.size())]);
                }
                packets.push(Packet {
                    structure_id: packet_structure.id,
                    field_data,
                    field_meta_data: vec![],
                });

                // This points to the index *after* the packet ends.
                last_successful_match_end_index =
                    Some(packet_start_index + packet_structure.size());
            }
        }

        // Throw away any garbage data that remains so that it does not have to be re-parsed
        let last_parsed_index = max(
            self.unparsed_data.len().checked_sub(packet_structure_manager.maximum_packet_structure_size).unwrap_or(0),
            last_successful_match_end_index.unwrap_or(0),
        );
        if print_flag {
            println!("LPI: {}", last_parsed_index);
        }
        self.unparsed_data.drain(0..last_parsed_index);
        packets
    }
}

//checks if the delimiter of a packet can be found in the given data
fn is_delimiter_match(data: &Vec<u8>, start_index: usize, delimiter_identifier: &Vec<u8>,print_flag: bool) -> bool {
    if start_index + delimiter_identifier.len() - 1 >= data.len() {
        return false;
    }

    for j in 0..delimiter_identifier.len() {
        if print_flag {
            print!("{:02X?}",data[start_index + j]);
            println!("{:02X?}",delimiter_identifier[j]);
        }
        if data[start_index + j] != delimiter_identifier[j] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::models::packet_structure::PacketStructure;

    use super::*;//lets the unit tests use everything in this file
    /// test for basic packet recognition and parsing
    #[test]
    fn test_basic_parsing(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
            packet_crc: vec![]
        };
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5]);
        let id = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut packet_parser = SerialPacketParser::default();
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
                    0x1E,0xAB,0x11,0xCA];
        packet_parser.push_data(&data,false);
        let parsed = packet_parser.parse_packets(&packet_structure_manager,false);
        assert_eq!(parsed[0].structure_id,id);//does the packet have the right ID?
        assert_eq!(parsed[0].field_data[0],PacketFieldValue::SignedLong(0));//does the data parse correctly?
        assert_eq!(parsed[0].field_data[1],PacketFieldValue::UnsignedShort(1));
        assert_eq!(parsed[0].field_data[2],PacketFieldValue::UnsignedShort(2));
        assert_eq!(parsed[0].field_data[3],PacketFieldValue::UnsignedByte(3));
        assert_eq!(parsed[0].field_data[4],PacketFieldValue::UnsignedByte(4));
    }
    
    /// test that data isn't mistaken for packets
    #[test]
    fn can_data_be_mistaken_for_delimiters(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
            packet_crc: vec![]
        };
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5]);
        packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut packet_parser = SerialPacketParser::default();
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
        let parsed = packet_parser.parse_packets(&packet_structure_manager,false);
        assert_eq!(parsed.len(),1);//is only the first packet parsed?
        assert_eq!(parsed[0].field_data[1],PacketFieldValue::UnsignedShort(1));//does some of the data still get parsed correctly?
        assert_eq!(parsed[0].field_data[2],PacketFieldValue::UnsignedShort(2));
        assert_eq!(parsed[0].field_data[3],PacketFieldValue::UnsignedByte(3));
        assert_eq!(parsed[0].field_data[4],PacketFieldValue::UnsignedByte(4));
    }

    /// test for packets of slightly longer or shorter length than expected
    #[test]
    fn bad_data_test(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0,
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
            packet_crc: vec![]
        };
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5]);
        packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut packet_parser = SerialPacketParser::default();
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
        let parsed = packet_parser.parse_packets(&packet_structure_manager,false);
        assert_eq!(parsed.len(),0);//did we accidentally parse any packets?
    }
    
    /// test consecutive packets
    #[test]
    fn consecutive_parsing_test(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
            packet_crc: vec![]
        };
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5]);
        packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut packet_parser = SerialPacketParser::default();
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
        let parsed = packet_parser.parse_packets(&packet_structure_manager,false);
        assert_eq!(parsed.len(),5);//did we catch all the packets?
        assert_eq!(parsed[2].field_data[1],PacketFieldValue::UnsignedShort(1));//did we parse the first group of packets correctly
        assert_eq!(parsed[2].field_data[2],PacketFieldValue::UnsignedShort(2));
        assert_eq!(parsed[2].field_data[3],PacketFieldValue::UnsignedByte(3));
        assert_eq!(parsed[2].field_data[4],PacketFieldValue::UnsignedByte(4));
        assert_eq!(parsed[4].field_data[1],PacketFieldValue::UnsignedShort(5));//did we parse the second group of packets correctly
        assert_eq!(parsed[4].field_data[2],PacketFieldValue::UnsignedShort(6));
        assert_eq!(parsed[4].field_data[3],PacketFieldValue::UnsignedByte(7));
        assert_eq!(parsed[4].field_data[4],PacketFieldValue::UnsignedByte(8));
    }
    
    /// test parsing with multiple packet structures, make sure to look at ID's
    #[test]
    fn multiple_structures(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
            packet_crc: vec![]
        };
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5]);
        let id1 = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut wacky_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Wacky Structure"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
            packet_crc: vec![],
        };
        wacky_structure.ez_make("i16 fa1a1a1a u8 u64", &["";3]);
        let id2 = packet_structure_manager.register_packet_structure(&mut wacky_structure).unwrap();
        let mut packet_parser = SerialPacketParser::default();
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
        let parsed = packet_parser.parse_packets(&packet_structure_manager,true);
        assert_eq!(parsed.len(),2); // are packets parsed?
        assert_eq!(parsed[0].structure_id,id1);//does the packet have the right ID?
        assert_eq!(parsed[0].field_data[0],PacketFieldValue::SignedLong(0));//does the data parse correctly?
        assert_eq!(parsed[0].field_data[1],PacketFieldValue::UnsignedShort(1));
        assert_eq!(parsed[0].field_data[2],PacketFieldValue::UnsignedShort(2));
        assert_eq!(parsed[0].field_data[3],PacketFieldValue::UnsignedByte(3));
        assert_eq!(parsed[0].field_data[4],PacketFieldValue::UnsignedByte(4));
        assert_eq!(parsed[1].structure_id,id2);//does the packet have the right ID?
        assert_eq!(parsed[1].field_data[0],PacketFieldValue::SignedShort(1));//does the data parse correctly?
        assert_eq!(parsed[1].field_data[1],PacketFieldValue::UnsignedByte(2));
        assert_eq!(parsed[1].field_data[2],PacketFieldValue::UnsignedLong(3));
    }

    // test parsing with multiple ps's that have the same first delimiter
    #[test]
    fn same_first_delim(){
        let mut packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
            packet_crc: vec![]
        };
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8 _4 ca11ab1e", &["";5]);
        let id1 = packet_structure_manager.register_packet_structure(&mut p_structure).unwrap();
        let mut wacky_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure Variation"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
            packet_crc: vec![]
        };
        wacky_structure.ez_make("ba5eba11 0020 0008 i64 u32 i8 _4 deadbeef", &["";3]);
        let id2 = packet_structure_manager.register_packet_structure(&mut wacky_structure).unwrap();
        let mut packet_parser = SerialPacketParser::default();
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
        let parsed = packet_parser.parse_packets(&packet_structure_manager,true);
        assert_eq!(parsed.len(),2); // are packets parsed?
        assert_eq!(parsed[0].structure_id,id2);//does the packet have the right ID?
        assert_eq!(parsed[0].field_data[0],PacketFieldValue::SignedLong(5));//does the data parse correctly?
        assert_eq!(parsed[0].field_data[1],PacketFieldValue::UnsignedInteger(0x10000));
        assert_eq!(parsed[0].field_data[2],PacketFieldValue::SignedByte(3));
        assert_eq!(parsed[1].structure_id,id1);//does the packet have the right ID?
        assert_eq!(parsed[1].field_data[0],PacketFieldValue::SignedLong(0));//does the data parse correctly?
        assert_eq!(parsed[1].field_data[1],PacketFieldValue::UnsignedShort(1));
        assert_eq!(parsed[1].field_data[2],PacketFieldValue::UnsignedShort(2));
        assert_eq!(parsed[1].field_data[3],PacketFieldValue::UnsignedByte(3));
        assert_eq!(parsed[1].field_data[4],PacketFieldValue::UnsignedByte(4));
    }

    // test for when packets dont make it into the pushed data state
    #[test]
    fn delimiter_led_packet_half_in_buffer(){
        let packet_structure_manager = PacketStructureManager::default();
        let mut p_structure = PacketStructure {
            id: 0, // gets overridden
            name: String::from("Test Structure"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
            packet_crc: vec![]
        };
        p_structure.ez_make("ba5eba11 0010 0008 i64 u16 u16 u8 u8", &["";5]);
        let mut packet_parser = SerialPacketParser::default();
        let data = [0x11,0xBA,0x5E,0xBA,
                    0x10,0x00,
                    0x08,0x00,
                    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                    0x01,0x00,
                    0x02,0x00,
                    0x03];
        packet_parser.push_data(&data,false);
        let parsed = packet_parser.parse_packets(&packet_structure_manager,false);
        assert_eq!(parsed,vec![]);//did we not parse anything?
    }

    //todo:
    // test packet structures that dont start/end with delimiters
    // check for value edge cases(like stuff that causes unsafe subtraction)
}
