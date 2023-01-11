use std::cmp::{min, max};

#[derive(Default)]
pub struct PacketParser {

    packet_structures: Vec<PacketStructure>,
    minimum_packet_structure_size: usize,
    maximum_packet_structure_size: usize,

    unparsed_data: Vec<u8>,

}

#[derive(serde::Serialize)]
pub enum PacketStructureRegistrationError {
    NameAlreadyRegistered(usize),
    DelimitersAlreadyRegistered(usize),
}

impl PacketParser {

    pub fn register_packet_structure(&mut self, packet_structure: PacketStructure) -> Result<usize, PacketStructureRegistrationError> {
        for (index, registered_packet_structure) in self.packet_structures.iter().enumerate() {
            if *registered_packet_structure.name == packet_structure.name {
                return Err(PacketStructureRegistrationError::NameAlreadyRegistered(index));
            } else if registered_packet_structure.delimiters == packet_structure.delimiters {
                return Err(PacketStructureRegistrationError::DelimitersAlreadyRegistered(index));
            }
        }

        let packet_structure_size = packet_structure.size();

        self.packet_structures.push(packet_structure);
        self.minimum_packet_structure_size = min(self.minimum_packet_structure_size, packet_structure_size);
        self.maximum_packet_structure_size = max(self.maximum_packet_structure_size, packet_structure_size);

        Ok(self.packet_structures.len() - 1)
    }

    pub fn push_data(&mut self, data: &[u8]) {
        self.unparsed_data.extend(data);
    }

    pub fn parse_packets(&mut self) -> Vec<Packet> {
        let mut packets: Vec<Packet> = vec![];

        let mut last_successful_match_end_index: Option<usize> = None;

        let maximum_index = self.unparsed_data.len() - self.minimum_packet_structure_size;

        println!("Unparsed data: {:#?}", self.unparsed_data);

        for i in 0..maximum_index {
            // Try to find a matching packet for the data
            for j in 0..self.packet_structures.len() {
                let packet_structure = &self.packet_structures[j];

                println!("At index {}, matching structure {}", i, j);

                if !is_delimiter_match(&self.unparsed_data, i, &packet_structure.delimiters[0].identifier) {
                    println!("- First delimiter did not match");
                    continue;
                }

                if packet_structure.delimiters[0].offset_in_packet > i {
                    println!("- Packet starts before data begins!");
                    continue;
                }

                let packet_start_index = i - packet_structure.delimiters[0].offset_in_packet;

                if let Some(last_successful_match_end_index) = last_successful_match_end_index {
                    if packet_start_index <= last_successful_match_end_index {
                        // The current packet cannot overlap with a previous one
                        println!("- Overlaps with previous packet");
                        continue;
                    }
                }

                let mut is_remaining_delimiters_matched = true;

                for delimiter in &packet_structure.delimiters[1..] {
                    let delimiter_start_index = packet_start_index + delimiter.offset_in_packet;
                    if !is_delimiter_match(&self.unparsed_data, delimiter_start_index, &delimiter.identifier) {
                        is_remaining_delimiters_matched = false;
                        break;
                    }
                }

                if !is_remaining_delimiters_matched {
                    println!("- Remaining delimiters did not match");
                    continue;
                }

                // The packet is a match, parse its data
                let mut field_data: Vec<PacketFieldValue> = vec![PacketFieldValue::UnsignedByte(0); packet_structure.fields.len()];
                let mut timestamp: Option<i64> = None;

                for k in 0..packet_structure.fields.len() {
                    let field = &packet_structure.fields[k];
                    let field_start_index = packet_start_index + field.offset_in_packet;

                    field_data[k] = field.r#type.parse(&self.unparsed_data[field_start_index..(field_start_index + field.r#type.size())]);

                    if field.metadata_type == PacketMetadataType::Timestamp {
                        if let PacketFieldValue::SignedLong(given_timestamp) = field_data[k] {
                            timestamp = Some(given_timestamp);
                        }
                    }
                }

                println!("- MATCHED!");

                packets.push(Packet {
                    structure_id: packet_structure.id,
                    field_data,
                    timestamp: timestamp.unwrap_or(chrono::offset::Utc::now().timestamp_millis()),
                });

                last_successful_match_end_index = Some(packet_start_index + packet_structure.size());
            }
        }

        // Throw away any garbage data that remains so that it does not have to be re-parsed
        let last_parsed_index = max(self.unparsed_data.len() - self.maximum_packet_structure_size, 
            last_successful_match_end_index.unwrap_or(usize::MIN));
        println!("LPI: {}", last_parsed_index);
        self.unparsed_data.drain(0..last_parsed_index);

        packets
    }

    pub fn set_field_name(&mut self, packet_structure_id: usize, field_index: usize, name: &str) {
        self.packet_structures[packet_structure_id].fields[field_index].name = name.to_string();
    }

    pub fn set_field_type(&mut self, packet_structure_id: usize, field_index: usize, r#type: PacketFieldType) {
        self.packet_structures[packet_structure_id].fields[field_index].r#type = r#type;
    }

}

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

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct Packet {
    structure_id: usize,
    field_data: Vec<PacketFieldValue>,
    timestamp: i64,
}

#[derive(PartialEq)]
#[derive(serde::Deserialize)]
pub struct PacketStructure {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) fields: Vec<PacketField>,
    pub(crate) delimiters: Vec<PacketDelimiter>,
}

impl PacketStructure {

    pub fn size(&self) -> usize {
        let mut size: usize = 0;

        for field in &self.fields {
            size += field.r#type.size();
        }

        for delimiter in &self.delimiters {
            size += delimiter.identifier.len();
        }

        size
    }

}

#[derive(PartialEq)]
#[derive(serde::Deserialize)]
pub struct PacketField {
    pub(crate) name: String,
    pub(crate) r#type: PacketFieldType,
    pub(crate) offset_in_packet: usize,
    pub(crate) metadata_type: PacketMetadataType,
}

#[derive(PartialEq)]
#[derive(serde::Deserialize)]
pub enum PacketFieldType {
    UnsignedByte,
    SignedByte,
    UnsignedShort,
    SignedShort,
    UnsignedInteger,
    SignedInteger,
    UnsignedLong,
    SignedLong,
    Float,
    Double
}

#[derive(PartialEq)]
#[derive(serde::Deserialize)]
pub enum PacketMetadataType {
    None,
    Timestamp,
}

#[derive(serde::Serialize)]
#[derive(Clone)]
#[derive(Debug)]
pub enum PacketFieldValue {
    UnsignedByte(u8),
    SignedByte(i8),
    UnsignedShort(u16),
    SignedShort(i16),
    UnsignedInteger(u32),
    SignedInteger(i32),
    UnsignedLong(u64),
    SignedLong(i64),
    Float(f32),
    Double(f64)
}

impl PacketFieldType {

    pub fn parse(&self, bytes: &[u8]) -> PacketFieldValue {
        match self {
            PacketFieldType::UnsignedByte => PacketFieldValue::UnsignedByte(u8::from_le_bytes(slice_to_fixed_size::<1>(bytes))),
            PacketFieldType::SignedByte => PacketFieldValue::SignedByte(i8::from_le_bytes(slice_to_fixed_size::<1>(bytes))),
            PacketFieldType::UnsignedShort => PacketFieldValue::UnsignedShort(u16::from_le_bytes(slice_to_fixed_size::<2>(bytes))),
            PacketFieldType::SignedShort => PacketFieldValue::SignedShort(i16::from_le_bytes(slice_to_fixed_size::<2>(bytes))),
            PacketFieldType::UnsignedInteger => PacketFieldValue::UnsignedInteger(u32::from_le_bytes(slice_to_fixed_size::<4>(bytes))),
            PacketFieldType::SignedInteger => PacketFieldValue::SignedInteger(i32::from_le_bytes(slice_to_fixed_size::<4>(bytes))),
            PacketFieldType::UnsignedLong => PacketFieldValue::UnsignedLong(u64::from_le_bytes(slice_to_fixed_size::<8>(bytes))),
            PacketFieldType::SignedLong => PacketFieldValue::SignedLong(i64::from_le_bytes(slice_to_fixed_size::<8>(bytes))),
            PacketFieldType::Float => PacketFieldValue::Float(f32::from_le_bytes(slice_to_fixed_size::<4>(bytes))),
            PacketFieldType::Double => PacketFieldValue::Double(f64::from_le_bytes(slice_to_fixed_size::<8>(bytes))),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            PacketFieldType::UnsignedByte | PacketFieldType::SignedByte => 1,
            PacketFieldType::UnsignedShort | PacketFieldType::SignedShort => 2,
            PacketFieldType::UnsignedInteger | PacketFieldType::SignedInteger => 4,
            PacketFieldType::UnsignedLong | PacketFieldType::SignedLong => 8,
            PacketFieldType::Float => 4,
            PacketFieldType::Double => 8,
        }
    }

}

fn slice_to_fixed_size<const N: usize>(slice: &[u8]) -> [u8; N] {
    slice.try_into().expect("Given slice has incorrect length!")
}

#[derive(PartialEq)]
#[derive(serde::Deserialize)]
pub struct PacketDelimiter {
    pub(crate) name: String,
    pub(crate) identifier: Vec<u8>,
    pub(crate) offset_in_packet: usize,
}