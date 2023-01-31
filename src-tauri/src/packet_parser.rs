use std::cmp::{max, min};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct PacketViewModel {
    id: usize,
    name: String,
    components: Vec<PacketComponent>,
}

#[derive(Serialize, Clone)]
pub struct PacketGap {
    index: usize,
    size: usize,
    offset_in_packet: usize,
}

#[derive(Serialize, Clone)]
pub enum PacketComponent {
    Field(PacketField),
    Delimiter(PacketDelimiter),
    Gap(PacketGap),
}

pub struct PacketParser {
    packet_structures: Vec<PacketStructure>,
    minimum_packet_structure_size: usize,
    maximum_packet_structure_size: usize,

    pub(crate) packet_view_models: Vec<PacketViewModel>,

    unparsed_data: Vec<u8>,
}

impl Default for PacketParser {
    fn default() -> Self {
        let default_packet_structures = vec![PacketStructure {
            id: 0,
            name: String::from("Packet 1"),
            fields: vec![
                PacketField {
                    index: 0,
                    name: String::from("Field 1"),
                    offset_in_packet: 0,
                    r#type: PacketFieldType::UnsignedInteger,
                    metadata_type: PacketMetadataType::None,
                },
                PacketField {
                    index: 1,
                    name: String::from("Field 2"),
                    offset_in_packet: 7,
                    r#type: PacketFieldType::SignedByte,
                    metadata_type: PacketMetadataType::None,
                },
            ],
            delimiters: vec![PacketDelimiter {
                index: 0,
                name: String::from("Delimiter 1"),
                identifier: vec![0xFF, 0xAB, 0x21],
                offset_in_packet: 4,
            }],
        }];
        let default_packet_view_models = default_packet_structures
            .iter()
            .map(|packet_structure| Self::create_packet_view_model(packet_structure))
            .collect();

        Self {
            packet_structures: default_packet_structures,
            minimum_packet_structure_size: Default::default(),
            maximum_packet_structure_size: Default::default(),
            packet_view_models: default_packet_view_models,
            unparsed_data: Default::default(),
        }
    }
}

#[derive(Serialize)]
pub enum PacketStructureRegistrationError {
    NameAlreadyRegistered(usize),
    DelimitersAlreadyRegistered(usize),
}

impl PacketParser {
    fn create_packet_view_model(packet_structure: &PacketStructure) -> PacketViewModel {
        let mut components: Vec<PacketComponent> =
            Vec::with_capacity(packet_structure.delimiters.len() + packet_structure.fields.len());

        for field in &packet_structure.fields {
            components.push(PacketComponent::Field(field.clone()));
        }

        for delimiter in &packet_structure.delimiters {
            components.push(PacketComponent::Delimiter(delimiter.clone()));
        }

        components.sort_by(|lhs, rhs| lhs.get_offset_in_packet().cmp(&rhs.get_offset_in_packet()));

        let mut gap_index: usize = 0;

        for i in 0..(components.len() - 1) {
            let component = &components[i];

            let current_component_end = component.get_offset_in_packet() + component.len();
            let next_offset = components[i + 1].get_offset_in_packet();

            if current_component_end < next_offset {
                components.insert(
                    i + 1,
                    PacketComponent::Gap(PacketGap {
                        index: gap_index,
                        size: next_offset - current_component_end,
                        offset_in_packet: current_component_end,
                    }),
                );
                gap_index += 1;
            }
        }

        return PacketViewModel {
            id: packet_structure.id,
            name: packet_structure.name.clone(),
            components,
        };
    }

    pub fn register_packet_structure(
        &mut self,
        packet_structure: PacketStructure,
    ) -> Result<usize, PacketStructureRegistrationError> {
        for (index, registered_packet_structure) in self.packet_structures.iter().enumerate() {
            if *registered_packet_structure.name == packet_structure.name {
                return Err(PacketStructureRegistrationError::NameAlreadyRegistered(
                    index,
                ));
            } else if registered_packet_structure.delimiters == packet_structure.delimiters {
                return Err(PacketStructureRegistrationError::DelimitersAlreadyRegistered(index));
            }
        }

        self.packet_view_models
            .push(Self::create_packet_view_model(&packet_structure));

        let packet_structure_size = packet_structure.size();

        self.packet_structures.push(packet_structure);
        self.minimum_packet_structure_size =
            min(self.minimum_packet_structure_size, packet_structure_size);
        self.maximum_packet_structure_size =
            max(self.maximum_packet_structure_size, packet_structure_size);

        Ok(self.packet_structures.len() - 1)
    }

    pub fn push_data(&mut self, data: &[u8]) {
        self.unparsed_data.extend(data);
    }

    pub fn parse_packets(&mut self) -> Vec<Packet> {
        let mut packets: Vec<Packet> = vec![];

        let mut last_successful_match_end_index: Option<usize> = None;

        let maximum_index = self.unparsed_data.len() - self.minimum_packet_structure_size;

        // println!("Unparsed data: {:#?}", self.unparsed_data);

        for i in 0..maximum_index {
            // Try to find a matching packet for the data
            for j in 0..self.packet_structures.len() {
                let packet_structure = &self.packet_structures[j];

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
                    // println!("- Packet starts before data begins!");
                    continue;
                }

                let packet_start_index = i - packet_structure.delimiters[0].offset_in_packet;

                if let Some(last_successful_match_end_index) = last_successful_match_end_index {
                    if packet_start_index <= last_successful_match_end_index {
                        // The current packet cannot overlap with a previous one
                        // println!("- Overlaps with previous packet");
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
                    // println!("- Remaining delimiters did not match");
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

                // println!("- MATCHED!");

                packets.push(Packet {
                    structure_id: packet_structure.id,
                    field_data,
                    timestamp: timestamp.unwrap_or(chrono::offset::Utc::now().timestamp_millis()),
                });

                last_successful_match_end_index =
                    Some(packet_start_index + packet_structure.size());
            }
        }

        // Throw away any garbage data that remains so that it does not have to be re-parsed
        let last_parsed_index = max(
            self.unparsed_data.len() - self.maximum_packet_structure_size,
            last_successful_match_end_index.unwrap_or(usize::MIN),
        );
        println!("LPI: {}", last_parsed_index);
        self.unparsed_data.drain(0..last_parsed_index);

        packets
    }

    fn update_packet_view_model(&mut self, packet_structure_id: usize) -> Result<Vec<usize>, (Vec<usize>, String)> {
        self.packet_view_models[packet_structure_id] =
            Self::create_packet_view_model(&self.packet_structures[packet_structure_id]);
        Ok(vec![packet_structure_id])
    }

    pub fn set_field_name(
        &mut self,
        packet_structure_id: usize,
        field_index: usize,
        name: &str,
    ) -> Result<Vec<usize>, (Vec<usize>, String)> {
        self.packet_structures[packet_structure_id].fields[field_index].name = String::from(name);

        Self::update_packet_view_model(self, packet_structure_id)
    }

    fn shift_components_after(
        packet_structure: &mut PacketStructure,
        offset_diff: isize,
        minimum_offset: usize,
    ) {
        for field in &mut packet_structure.fields {
            if field.offset_in_packet > minimum_offset {
                field.offset_in_packet = field
                    .offset_in_packet
                    .checked_add_signed(offset_diff)
                    .expect("Packet component offset calculation failed!");
            }
        }

        for delimiter in &mut packet_structure.delimiters {
            if delimiter.offset_in_packet > minimum_offset {
                delimiter.offset_in_packet = delimiter
                    .offset_in_packet
                    .checked_add_signed(offset_diff)
                    .expect("Packet component offset calculation failed!");
            }
        }
    }

    pub fn set_field_type(
        &mut self,
        packet_structure_id: usize,
        field_index: usize,
        r#type: PacketFieldType,
    ) -> Result<Vec<usize>, (Vec<usize>, String)> {
        let packet_structure = &mut self.packet_structures[packet_structure_id];
        let packet_structure_fields = &mut packet_structure.fields;

        let minimum_offset: usize;
        let offset_diff: isize;
        {
            let field_to_modify = &mut packet_structure_fields[field_index];

            minimum_offset = field_to_modify.offset_in_packet;
            offset_diff = (r#type.size() as isize) - (field_to_modify.r#type.size() as isize);

            field_to_modify.r#type = r#type;
        }

        Self::shift_components_after(packet_structure, offset_diff, minimum_offset);

        Self::update_packet_view_model(self, packet_structure_id)
    }

    pub fn set_field_metadata_type(
        &mut self,
        packet_structure_id: usize,
        field_index: usize,
        metadata_type: PacketMetadataType,
    ) -> Result<Vec<usize>, (Vec<usize>, String)> {
        self.packet_structures[packet_structure_id].fields[field_index].metadata_type =
            metadata_type;

        Self::update_packet_view_model(self, packet_structure_id)
    }

    pub fn set_delimiter_name(
        &mut self,
        packet_structure_id: usize,
        delimtier_index: usize,
        name: &str,
    ) -> Result<Vec<usize>, (Vec<usize>, String)> {
        self.packet_structures[packet_structure_id].delimiters[delimtier_index].name =
            String::from(name);

        Self::update_packet_view_model(self, packet_structure_id)
    }

    // Adapted from https://codereview.stackexchange.com/a/201699
    fn get_hex_char_value(hex_char: u8) -> Option<u8> {
        match hex_char {
            b'0'..=b'9' => Some(hex_char - b'0'),
            b'a'..=b'f' => Some(hex_char - b'a' + 10),
            b'A'..=b'F' => Some(hex_char - b'A' + 10),
            _ => None,
        }
    }

    fn parse_hex(hex_string: &str) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::with_capacity((hex_string.len() + 1) / 2);

        let hex_string_bytes = hex_string.as_bytes();

        for i in (0..hex_string_bytes.len()).step_by(2) {
            let first_hex_value = Self::get_hex_char_value(hex_string_bytes[i]);
            if first_hex_value.is_none() {
                return Err("Invalid hex character!");
            }
            let next_hex_value = if i + 1 < hex_string_bytes.len() {
                let next_hex_value = Self::get_hex_char_value(hex_string_bytes[i + 1]);
                if next_hex_value.is_none() {
                    return Err("Invalid hex character!");
                }
                next_hex_value
            } else {
                None
            };

            bytes.push(first_hex_value.unwrap() << 4 | next_hex_value.unwrap_or(0))
        }

        Ok(bytes)
    }

    pub fn set_delimiter_identifier(
        &mut self,
        packet_structure_id: usize,
        delimtier_index: usize,
        identifier: &str,
    ) -> Result<Vec<usize>, (Vec<usize>, String)> {
        let hex_array = Self::parse_hex(identifier);
        if hex_array.is_err() {
            return Err((
                vec![packet_structure_id],
                String::from(hex_array.unwrap_err()),
            ));
        }

        // TODO: do not allow empty hex

        let packet_structure = &mut self.packet_structures[packet_structure_id];

        let offset_diff: isize;
        let minimum_offset: usize;

        {
            let delimiter = &mut packet_structure.delimiters[delimtier_index];

            offset_diff =
                hex_array.as_ref().unwrap().len() as isize - delimiter.identifier.len() as isize;
            minimum_offset = delimiter.offset_in_packet;

            delimiter.identifier = hex_array.unwrap();
        }

        Self::shift_components_after(packet_structure, offset_diff, minimum_offset);

        // TODO handle conflicts

        Self::update_packet_view_model(self, packet_structure_id)
    }

    pub fn set_gap_size(
        &mut self,
        packet_structure_id: usize,
        gap_index: usize,
        gap_size: usize,
    ) -> Result<Vec<usize>, (Vec<usize>, String)> {
        let components = &mut self.packet_view_models[packet_structure_id].components;

        // Find the gap with the given index
        let mut found_gap_index: usize = 0;
        for i in 0..(components.len() - 1) {
            let component = &components[i];

            let current_component_end = component.get_offset_in_packet() + component.len();
            let next_offset = components[i + 1].get_offset_in_packet();

            if current_component_end < next_offset {
                if found_gap_index == gap_index {
                    // Shift all packet components after the current one by the change in the gap size
                    let gap_delta = -((next_offset - current_component_end - gap_size) as isize);

                    for j in (i + 1)..components.len() {
                        let new_offset =
                            (components[j].get_offset_in_packet() as isize + gap_delta) as usize;
                        components[j].set_offset_in_packet(new_offset);
                    }

                    return Ok(vec![packet_structure_id]);
                }

                found_gap_index += 1;
            }
        }

        Err((
            vec![],
            String::from("Failed to find gap with the given index!"),
        ))
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

#[derive(Serialize, Debug)]
pub struct Packet {
    structure_id: usize,
    field_data: Vec<PacketFieldValue>,
    timestamp: i64,
}

#[derive(PartialEq, Deserialize)]
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

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct PacketField {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) r#type: PacketFieldType,
    pub(crate) offset_in_packet: usize,
    pub(crate) metadata_type: PacketMetadataType,
}

#[repr(u8)]
#[derive(PartialEq, Serialize, Deserialize, Clone, Copy, Debug)]
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
    Double,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PacketMetadataType {
    None,
    Timestamp,
}

#[derive(Serialize, Clone, Debug)]
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
    Double(f64),
}

impl PacketFieldType {
    pub fn parse(&self, bytes: &[u8]) -> PacketFieldValue {
        match self {
            PacketFieldType::UnsignedByte => {
                PacketFieldValue::UnsignedByte(u8::from_le_bytes(slice_to_fixed_size::<1>(bytes)))
            }
            PacketFieldType::SignedByte => {
                PacketFieldValue::SignedByte(i8::from_le_bytes(slice_to_fixed_size::<1>(bytes)))
            }
            PacketFieldType::UnsignedShort => {
                PacketFieldValue::UnsignedShort(u16::from_le_bytes(slice_to_fixed_size::<2>(bytes)))
            }
            PacketFieldType::SignedShort => {
                PacketFieldValue::SignedShort(i16::from_le_bytes(slice_to_fixed_size::<2>(bytes)))
            }
            PacketFieldType::UnsignedInteger => PacketFieldValue::UnsignedInteger(
                u32::from_le_bytes(slice_to_fixed_size::<4>(bytes)),
            ),
            PacketFieldType::SignedInteger => {
                PacketFieldValue::SignedInteger(i32::from_le_bytes(slice_to_fixed_size::<4>(bytes)))
            }
            PacketFieldType::UnsignedLong => {
                PacketFieldValue::UnsignedLong(u64::from_le_bytes(slice_to_fixed_size::<8>(bytes)))
            }
            PacketFieldType::SignedLong => {
                PacketFieldValue::SignedLong(i64::from_le_bytes(slice_to_fixed_size::<8>(bytes)))
            }
            PacketFieldType::Float => {
                PacketFieldValue::Float(f32::from_le_bytes(slice_to_fixed_size::<4>(bytes)))
            }
            PacketFieldType::Double => {
                PacketFieldValue::Double(f64::from_le_bytes(slice_to_fixed_size::<8>(bytes)))
            }
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

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct PacketDelimiter {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) identifier: Vec<u8>,
    pub(crate) offset_in_packet: usize,
}

impl PacketComponent {
    fn get_offset_in_packet(&self) -> usize {
        match self {
            PacketComponent::Field(field) => field.offset_in_packet,
            PacketComponent::Delimiter(delimiter) => delimiter.offset_in_packet,
            PacketComponent::Gap(gap) => gap.offset_in_packet,
        }
    }

    fn set_offset_in_packet(&mut self, offset_in_packet: usize) {
        match self {
            PacketComponent::Field(field) => field.offset_in_packet = offset_in_packet,
            PacketComponent::Delimiter(delimiter) => delimiter.offset_in_packet = offset_in_packet,
            PacketComponent::Gap(gap) => gap.offset_in_packet = offset_in_packet,
        }
    }

    fn len(&self) -> usize {
        match self {
            PacketComponent::Field(field) => field.r#type.size(),
            PacketComponent::Delimiter(delimiter) => delimiter.identifier.len(),
            PacketComponent::Gap(gap) => gap.size,
        }
    }
}
