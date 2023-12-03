use std::cmp::max;

use crate::{
    models::packet::{Packet, PacketFieldValue},
    models::packet_structure::PacketMetadataType,
    packet_structure_manager::PacketStructureManager,
};

#[derive(Default)]
pub struct PacketParser {
    unparsed_data: Vec<u8>,
}

/// responsible converting raw data to packets
impl PacketParser {
    //adds new unparsed data
    pub fn push_data(&mut self, data: &[u8]) {
        self.unparsed_data.extend(data);
        //println!("Unparsed data: {:02X?}", self.unparsed_data);
    }

    /// processes the raw data queue, returning a Vector(aka. array) of the proccessesed packets
    pub fn parse_packets(
        &mut self,
        packet_structure_manager: &PacketStructureManager, //maybe this could just be a list for easier readability
    ) -> Vec<Packet> {
        //println!("Unparsed data length: {}", self.unparsed_data.len());
        let mut packets: Vec<Packet> = vec![];

        let mut last_successful_match_end_index: Option<usize> = None;

        let maximum_index =
            self.unparsed_data.len().checked_sub(packet_structure_manager.minimum_packet_structure_size).and_then(|min_index| Some(min_index + 1)).unwrap_or(0);

        for i in 0..maximum_index {
            // Try to find a matching packet for the data
            for j in 0..packet_structure_manager.packet_structures.len() {
                let packet_structure = &packet_structure_manager.packet_structures[j];

                // println!("At index {}, matching structure {}", i, j);

                if !is_delimiter_match(
                    &self.unparsed_data,
                    i,
                    &packet_structure.delimiters[0].identifier,
                ) {
                    // println!("- First delimiter did not match");
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
                        // The current packet cannot overlap with a previous one
                        println!("- Overlaps with previous packet");
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
                    ) {
                        is_remaining_delimiters_matched = false;
                        break;
                    }
                }

                if !is_remaining_delimiters_matched {
                    //println!("- Remaining delimiters did not match");
                    continue;
                }

                // The packet is a match, parse its data
                let mut field_data: Vec<PacketFieldValue> =
                    vec![PacketFieldValue::UnsignedByte(0); packet_structure.fields.len()];
                let mut timestamp: Option<i64> = None;

                for k in 0..packet_structure.fields.len() {
                    let field = &packet_structure.fields[k];
                    let field_start_index = packet_start_index + field.offset_in_packet;

                    field_data[k] = field.r#type.parse(
                        &self.unparsed_data
                            [field_start_index..(field_start_index + field.r#type.size())],
                    );

                    if field.metadata_type == PacketMetadataType::Timestamp {
                        if let PacketFieldValue::SignedLong(given_timestamp) = field_data[k] {
                            timestamp = Some(given_timestamp);
                        }
                    }
                }

                //println!("MATCHED: {:02X?}", &self.unparsed_data[packet_start_index..(packet_start_index + packet_structure.size())]);

                packets.push(Packet {
                    structure_id: packet_structure.id,
                    field_data,
                    timestamp: timestamp.unwrap_or(chrono::offset::Utc::now().timestamp_millis()),
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
        //println!("LPI: {}", last_parsed_index);
        self.unparsed_data.drain(0..last_parsed_index);

        packets
    }
}

//checks if the delimiter of a packet can be found in the given data
fn is_delimiter_match(data: &Vec<u8>, start_index: usize, delimiter_identifier: &Vec<u8>) -> bool {
    if start_index + delimiter_identifier.len() - 1 >= data.len() {
        return false;
    }

    for j in 0..delimiter_identifier.len() {
        if data[start_index + j] != delimiter_identifier[j] {
            return false;
        }
    }

    true
}
