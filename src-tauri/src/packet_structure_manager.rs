use std::{
    cmp::{max, min},
    vec,
};

use crate::packet_structure::{
    PacketDelimiter, PacketField, PacketFieldType, PacketMetadataType, PacketStructure,
};

#[readonly::make]
#[derive(Default)]
pub struct PacketStructureManager {
    pub(crate) packet_structures: Vec<PacketStructure>,
    pub(crate) minimum_packet_structure_size: usize,
    pub(crate) maximum_packet_structure_size: usize,
}

pub enum PacketStructureRegistrationError {
    NameAlreadyRegistered(usize),
    DelimitersAlreadyRegistered(usize),
}

#[derive(Clone)]
enum PacketFieldOrDelimiter {
    Field(PacketField),
    Delimiter(PacketDelimiter),
}

impl PacketFieldOrDelimiter {
    pub fn end_offset(&self) -> usize {
        match self {
            PacketFieldOrDelimiter::Field(field) => field.offset_in_packet + field.r#type.size(),
            PacketFieldOrDelimiter::Delimiter(delimiter) => {
                delimiter.offset_in_packet + delimiter.identifier.len()
            }
        }
    }
}

pub enum SetDelimiterIdentifierError {
    InvalidHexadecimalString(String),
    IdentifierCollision(Vec<usize>),
}

impl PacketStructureManager {
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

        let packet_structure_size = packet_structure.size();

        self.packet_structures.push(packet_structure);
        self.minimum_packet_structure_size =
            min(self.minimum_packet_structure_size, packet_structure_size);
        self.maximum_packet_structure_size =
            max(self.maximum_packet_structure_size, packet_structure_size);

        Ok(self.packet_structures.len() - 1)
    }

    pub fn set_field_name(&mut self, packet_structure_id: usize, field_index: usize, name: &str) {
        self.packet_structures[packet_structure_id].fields[field_index].name = String::from(name);
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
    ) {
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
    }

    pub fn set_field_metadata_type(
        &mut self,
        packet_structure_id: usize,
        field_index: usize,
        metadata_type: PacketMetadataType,
    ) {
        self.packet_structures[packet_structure_id].fields[field_index].metadata_type =
            metadata_type;
    }

    pub fn set_delimiter_name(
        &mut self,
        packet_structure_id: usize,
        delimtier_index: usize,
        name: &str,
    ) {
        self.packet_structures[packet_structure_id].delimiters[delimtier_index].name =
            String::from(name);
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
    ) -> Result<(), SetDelimiterIdentifierError> {
        // Ensure new identifier is not empty
        if identifier.len() == 0 {
            return Err(SetDelimiterIdentifierError::InvalidHexadecimalString(
                String::from("Identifier must not be empty!"),
            ));
        }

        // Get array of bytes from the given string
        let hex_array = Self::parse_hex(identifier);
        // Ensure the given string is valid a hexadecimal string
        if hex_array.is_err() {
            return Err(SetDelimiterIdentifierError::InvalidHexadecimalString(
                String::from(hex_array.unwrap_err()),
            ));
        }

        let mut delimiters: Vec<PacketDelimiter>;
        let delimiter_offset: usize;
        let offset_diff: isize;
        {
            let packet_structure = &mut self.packet_structures[packet_structure_id];
            // Clone the delimiters so that the change to the packet structure is not immediately applied;
            // this makes handling an identifier collision easier later
            delimiters = packet_structure.delimiters.clone();

            let delimiter = &mut delimiters[delimtier_index];
            // Calculate the change in offset the following packet fields and delimiters will have
            offset_diff =
                hex_array.as_ref().unwrap().len() as isize - delimiter.identifier.len() as isize;
            delimiter.identifier = hex_array.unwrap();
            delimiter_offset = delimiter.offset_in_packet;
        }

        // Check for collisions with other packet structure identifiers
        let mut identifier_collisions = Vec::new();

        for other_packet_structure in &self.packet_structures {
            if other_packet_structure.id == packet_structure_id {
                continue;
            }

            if other_packet_structure.delimiters == delimiters {
                identifier_collisions.push(other_packet_structure.id);
            }
        }

        if !identifier_collisions.is_empty() {
            return Err(SetDelimiterIdentifierError::IdentifierCollision(
                identifier_collisions,
            ));
        }

        // Apply the change
        let packet_structure = &mut self.packet_structures[packet_structure_id];
        packet_structure.delimiters = delimiters;

        Self::shift_components_after(packet_structure, offset_diff, delimiter_offset);

        Ok(())
    }

    fn find_field_or_delimiter_with_offset(
        &self,
        packet_structure_id: usize,
        offset_in_packet: usize,
    ) -> Option<PacketFieldOrDelimiter> {
        for field in &self.packet_structures[packet_structure_id].fields {
            if field.offset_in_packet == offset_in_packet {
                return Some(PacketFieldOrDelimiter::Field(field.clone()));
            }
        }

        for delimiter in &self.packet_structures[packet_structure_id].delimiters {
            if delimiter.offset_in_packet == offset_in_packet {
                return Some(PacketFieldOrDelimiter::Delimiter(delimiter.clone()));
            }
        }

        return None;
    }

    pub fn set_gap_size(&mut self, packet_structure_id: usize, gap_index: usize, gap_size: usize) {
        // Find the gap with the given index
        let mut found_gap_index: usize = 0;

        let mut previous_field_or_delimiter: Option<PacketFieldOrDelimiter> = None;
        let mut maybe_field_or_delimiter =
            self.find_field_or_delimiter_with_offset(packet_structure_id, 0);

        while let Some(field_or_delimiter) = maybe_field_or_delimiter {
            let previous_field_or_delimiter_end = if previous_field_or_delimiter.is_some() {
                previous_field_or_delimiter.unwrap().end_offset()
            } else {
                0
            };
            let extra_space = field_or_delimiter.end_offset() - previous_field_or_delimiter_end;

            if extra_space > 0 {
                if gap_index == found_gap_index {
                    // Shift all packet components after the current one by the change in the gap size
                    let gap_delta: isize = gap_size as isize - extra_space as isize;

                    Self::shift_components_after(
                        &mut self.packet_structures[packet_structure_id],
                        gap_delta,
                        previous_field_or_delimiter_end,
                    );

                    return;
                }

                found_gap_index += 1;
            }

            previous_field_or_delimiter = Some(field_or_delimiter.clone());
            maybe_field_or_delimiter = self.find_field_or_delimiter_with_offset(
                packet_structure_id,
                field_or_delimiter.end_offset(),
            )
        }
    }

    pub fn add_field(&mut self, packet_structure_id: usize) {
        let packet_field_count = self.packet_structures[packet_structure_id].fields.len();
        let end_of_packet = self.packet_structures[packet_structure_id].size();

        self.packet_structures[packet_structure_id]
            .fields
            .push(PacketField {
                index: packet_field_count,
                metadata_type: PacketMetadataType::None,
                name: format!("Field {}", (packet_field_count + 1)),
                offset_in_packet: end_of_packet,
                r#type: PacketFieldType::UnsignedInteger,
            });
    }

    pub fn add_delimiter(&mut self, packet_structure_id: usize) {
        let packet_delimiter_count = self.packet_structures[packet_structure_id].delimiters.len();
        let end_of_packet = self.packet_structures[packet_structure_id].size();

        self.packet_structures[packet_structure_id]
            .delimiters
            .push(PacketDelimiter {
                index: packet_delimiter_count,
                name: format!("Delimiter {}", packet_delimiter_count + 1),
                identifier: vec![0xFF],
                offset_in_packet: end_of_packet,
            });
    }

    pub fn add_gap_after(
        &mut self,
        packet_structure_id: usize,
        is_field: bool,
        component_index: usize,
    ) {
        let packet_structure = &mut self.packet_structures[packet_structure_id];

        let minimum_offset = if is_field {
            packet_structure.fields[component_index].offset_in_packet
        } else {
            packet_structure.delimiters[component_index].offset_in_packet
        };

        Self::shift_components_after(packet_structure, 1, minimum_offset);
    }
}
