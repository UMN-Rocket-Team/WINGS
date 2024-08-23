use std::{
    cmp::{max, min},
    vec,
};

use serde::{Deserialize, Serialize};

use crate::{
    models::packet_structure::{
        PacketDelimiter, PacketField, PacketFieldType, PacketMetadataType, PacketStructure,
    },
    models::packet_view_model::PacketComponentType,
};

/// Represents all possible errors that can be encountered when managing packet structures.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Contains the ID that was asked for
    PacketDoesNotExist(usize),
    /// Contains the ID of the packet that already uses the name
    NameAlreadyRegistered(usize),
    /// Contains the ID of the packet that already uses these delimiters
    DelimitersAlreadyRegistered(usize),
    FieldOffsetOverflow,
    DelimiterOffsetOverflow,
    /// Contains the character that was invalid
    InvalidHexCharacter(char),
    EmptyDelimiterIdentifier,
    /// Contains a list of packet IDs that collide
    DelimiterIdentifierCollision(Vec<usize>),
    CannotDeleteLastField,
    CannotDeleteLastDelimiter,
    NoComponents,
    GapEndOverflow
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Self::PacketDoesNotExist(id) => format!("Packet {id} does not exist"),
            Self::NameAlreadyRegistered(name) => format!("Packet with name {name} already exists"),
            Self::DelimitersAlreadyRegistered(id) => format!("Delimiters are already registered by packet ID {id}"),
            Self::FieldOffsetOverflow => "Field offset overflow".to_string(),
            Self::DelimiterOffsetOverflow => "Delimiter offset overflow".to_string(),
            Self::InvalidHexCharacter(char) => format!("{char} is an invalid hex character"),
            Self::EmptyDelimiterIdentifier => format!("Delimiter identifier cannot be empty"),
            Self::DelimiterIdentifierCollision(ids) => format!("Delimiter identifiers collides with packet IDs {:?}", ids),
            Self::CannotDeleteLastField => "Cannot delete last field".to_string(),
            Self::CannotDeleteLastDelimiter => "Cannot delete last delimiter".to_string(),
            Self::NoComponents => "No components".to_string(),
            Self::GapEndOverflow => "Gap end overflow".to_string()
        }
    }
}

/// A packet structure manager is an object that contains all the packets the app is dealing with, this makes them easier to use them from other threads and handle errors
#[readonly::make]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PacketStructureManager {
    pub(crate) packet_structures: Vec<PacketStructure>,

    // These variables are used for parsing packets
    pub(crate) minimum_packet_structure_size: usize,
    pub(crate) maximum_packet_structure_size: usize,
    pub(crate) maximum_first_delimiter: usize,

    // Every packet structure is given an increasing ID
    next_packet_id: usize,
}

impl Default for PacketStructureManager {
    fn default() -> Self {
        Self {
            packet_structures: Default::default(),
            minimum_packet_structure_size: usize::MAX,
            maximum_packet_structure_size: 0,
            maximum_first_delimiter: 0,

            // We can start IDs from anywhere, but we start from 1 so that any code that
            // accidentally assumes IDs are an array index will be more likely to break
            // early.
            next_packet_id: 1
        }
    }
}

impl PacketStructureManager {
    /// Takes the given PacketStructure and makes a copy within the manager for future use
    /// Note: this also needs to be unit tested
    pub fn register_packet_structure(
        &mut self,
        packet_structure: &mut PacketStructure,
    ) -> Result<usize, Error> {
        for registered_packet_structure in self.packet_structures.iter() {
            if *registered_packet_structure.name == packet_structure.name {
                return Err(Error::NameAlreadyRegistered(registered_packet_structure.id));
            } else if registered_packet_structure.delimiters == packet_structure.delimiters {
                return Err(Error::DelimitersAlreadyRegistered(registered_packet_structure.id));
            }
        }

        packet_structure.id = self.next_packet_id;
        self.next_packet_id += 1;

        self.packet_structures.push(packet_structure.clone());
        self.update_tracked_values();
        Ok(packet_structure.id)
    }

    /// Get a immutable borrow to a packet structure by its ID.
    /// This is necessary because the IDs are **not** list indexes.
    pub fn get_packet_structure(
        &self,
        packet_structure_id: usize
    ) -> Result<&PacketStructure, Error> {
        for packet_structure in self.packet_structures.iter() {
            if packet_structure.id == packet_structure_id {
                return Ok(packet_structure);
            }
        }
        return Err(Error::PacketDoesNotExist(packet_structure_id));
    }

    /// Get a mutable borrow to a packet structure by its ID.
    /// This is necessary because the IDs are **not** list indexes.
    pub fn get_packet_structure_mut(
        &mut self,
        packet_structure_id: usize
    ) -> Result<&mut PacketStructure, Error> {
        for packet_structure in self.packet_structures.iter_mut() {
            if packet_structure.id == packet_structure_id {
                return Ok(packet_structure);
            }
        }
        return Err(Error::PacketDoesNotExist(packet_structure_id));
    }

    /// Sets the name of a specific packet structure
    pub fn set_packet_name(&mut self, packet_structure_id: usize, name: &str) -> Result<(), Error> {
        let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;
        packet_structure.name = String::from(name);
        Ok(())
    }

    /// Sets the name of a specific field within a packet structure
    pub fn set_field_name(&mut self, packet_structure_id: usize, field_index: usize, name: &str) -> Result<(), Error> {
        let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;
        packet_structure.fields[field_index].name = String::from(name);
        Ok(())
    }

    /// Shifts all components after a specified point in the packet structure
    /// For example:
    /// Before:
    /// [][][][][][][]
    /// After:
    /// [][][]  [][][][]
    ///
    /// offset_diff is the size we want to shift by
    /// minimum_offset is where the shifting will start
    fn shift_components_after(
        packet_structure: &mut PacketStructure,
        offset_diff: isize,
        minimum_offset: usize,
    ) -> Result<(), Error> {
        for field in &mut packet_structure.fields {
            if field.offset_in_packet > minimum_offset {
                field.offset_in_packet = match field.offset_in_packet.checked_add_signed(offset_diff) {
                    Some(n) => n,
                    None => {
                        return Err(Error::FieldOffsetOverflow);
                    }
                };
            }
        }

        for delimiter in &mut packet_structure.delimiters {
            if delimiter.offset_in_packet > minimum_offset {
                delimiter.offset_in_packet = match delimiter.offset_in_packet.checked_add_signed(offset_diff) {
                    Some(n) => n,
                    None => {
                        return Err(Error::DelimiterOffsetOverflow);
                    }
                };
            }
        }

        Ok(())
    }

    /// Sets the data type to expect for a field, and adjusts all following packets to make
    /// room for the new data if needed.
    pub fn set_field_type(
        &mut self,
        packet_structure_id: usize,
        field_index: usize,
        r#type: PacketFieldType,
    ) -> Result<(), Error> {
        let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;

        let minimum_offset: usize;
        let offset_diff: isize;
        {
            let field_to_modify = &mut packet_structure.fields[field_index];

            minimum_offset = field_to_modify.offset_in_packet;
            offset_diff = (r#type.size() as isize) - (field_to_modify.r#type.size() as isize);

            field_to_modify.r#type = r#type;
        }

        Self::shift_components_after(packet_structure, offset_diff, minimum_offset)?;

        Ok(())
    }

    /// Set the metadata type for a field of a packet structure.
    pub fn set_field_metadata_type(
        &mut self,
        _packet_structure_id: usize,
        _field_index: usize,
        _metadata_type: PacketMetadataType,
    ) -> Result<(), Error> {
        // Deprecated old method; to be replaced
        Ok(())
    }

    /// Set the name of a delimiter.
    pub fn set_delimiter_name(
        &mut self,
        packet_structure_id: usize,
        delimiter_index: usize,
        name: &str,
    ) -> Result<(), Error> {
        let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;
        packet_structure.delimiters[delimiter_index].name = String::from(name);
        Ok(())
    }

    /// Adapted from https://codereview.stackexchange.com/a/201699
    fn get_hex_char_value(hex_char: u8) -> Option<u8> {
        match hex_char {
            b'0'..=b'9' => Some(hex_char - b'0'),
            b'a'..=b'f' => Some(hex_char - b'a' + 10),
            b'A'..=b'F' => Some(hex_char - b'A' + 10),
            _ => None,
        }
    }

    /// Takes a hexadecimal string and returns its binary equivalent
    /// (currently used primarily to decode the user input for delimiter identifiers)
    fn parse_hex(hex_string: &str) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::with_capacity((hex_string.len() + 1) / 2);

        let hex_string_bytes = hex_string.as_bytes();

        for i in (0..hex_string_bytes.len()).step_by(2) {
            let first_hex_value = Self::get_hex_char_value(hex_string_bytes[i]);
            if first_hex_value.is_none() {
                return Err(Error::InvalidHexCharacter(hex_string_bytes[i] as char));
            }
            let next_hex_value = if i + 1 < hex_string_bytes.len() {
                let next_hex_value = Self::get_hex_char_value(hex_string_bytes[i + 1]);
                if next_hex_value.is_none() {
                    return Err(Error::InvalidHexCharacter(hex_string_bytes[i + 1] as char));
                }
                next_hex_value
            } else {
                None
            };

            bytes.push(first_hex_value.unwrap() << 4 | next_hex_value.unwrap_or(0))
        }

        Ok(bytes)
    }

    /// Checks if a a given set of delimiters conflicts with an existing packet
    /// The given ID will be ignored (presumably this is the packet being edited)
    fn check_for_identifier_collisions(
        packet_structures: &Vec<PacketStructure>,
        packet_structure_id: usize,
        delimiters: &Vec<PacketDelimiter>,
    ) -> Option<Vec<usize>> {
        let mut identifier_collisions = Vec::new();

        for other_packet_structure in packet_structures {
            if other_packet_structure.id == packet_structure_id {
                continue;
            }

            // This will break on some problematic delimiters, eg. ones that break up one
            // delimiter into multiple smaller ones.
            if other_packet_structure.delimiters == *delimiters {
                identifier_collisions.push(other_packet_structure.id);
            }
        }

        if identifier_collisions.is_empty() {
            return None
        }
        return Some(identifier_collisions);
    }

    /// Takes a delimiter and changes its identifier, then shifts all other fields as necessary to keep offset relative to new identifier
    pub fn set_delimiter_identifier(
        &mut self,
        packet_structure_id: usize,
        delimiter_index: usize,
        identifier: &str,
    ) -> Result<(), Error> {
        // Ensure new identifier is not empty
        if identifier.len() == 0 {
            return Err(Error::EmptyDelimiterIdentifier);
        }

        // Get array of bytes from the given string
        let hex_array = Self::parse_hex(identifier)?;

        let packet_structures = self.packet_structures.clone();
        let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;

        // Clone the delimiters so that the change to the packet structure is not immediately applied;
        // this makes handling an identifier collision easier later
        let mut delimiters = packet_structure.delimiters.clone();

        let delimiter = &mut delimiters[delimiter_index];
        // Calculate the change in offset the following packet fields and delimiters will have
        let offset_diff = hex_array.len() as isize - delimiter.identifier.len() as isize;
        delimiter.identifier = hex_array;
        let delimiter_offset = delimiter.offset_in_packet;

        // Check for collisions with other packet structure identifiers
        if let Some(colliding_ids) = Self::check_for_identifier_collisions(
            &packet_structures,
            packet_structure_id,
            &delimiters
        ) {
            return Err(Error::DelimiterIdentifierCollision(colliding_ids));
        }

        // Apply the change
        packet_structure.delimiters = delimiters;

        Self::shift_components_after(packet_structure, offset_diff, delimiter_offset)?;

        Ok(())
    }

    /// Changes the size of a gap inside a packet
    pub fn set_gap_size(
        &mut self,
        packet_structure_id: usize,
        gap_start: usize,
        new_gap_size: isize
    ) -> Result<(), Error> {
        let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;

        let field_offsets = packet_structure.fields.iter().map(|f| f.offset_in_packet);
        let delimiter_offsets = packet_structure.delimiters.iter().map(|d| d.offset_in_packet);
        let gap_end: usize = field_offsets
            .chain(delimiter_offsets)
            .filter(|offset| *offset >= gap_start)
            .min()
            .ok_or(Error::NoComponents)?
            .try_into()
            .map_err(|_| Error::GapEndOverflow)?;

        let gap_size: isize = (gap_end - gap_start)
            .try_into()
            .map_err(|_| Error::GapEndOverflow)?;

        Self::shift_components_after(packet_structure, new_gap_size - gap_size, gap_start)?;
        Ok(())
    }

    /// Creates a new field inside the end of the packet
    /// Needs to be reworked to add to a user-specified location
    pub fn add_field(&mut self, packet_structure_id: usize) -> Result<(), Error> {
        let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;

        let packet_field_count = packet_structure.fields.len();
        let end_of_packet = packet_structure.size();

        packet_structure
            .fields
            .push(PacketField {
                index: packet_field_count,
                name: format!("Field {}", (packet_field_count + 1)),
                offset_in_packet: end_of_packet,
                r#type: PacketFieldType::UnsignedInteger,
            });

        return Ok(());
    }

    /// Creates a new delimiter inside the end of the packet
    /// Needs to be reworked to add to a user-specified location
    pub fn add_delimiter(&mut self, packet_structure_id: usize) -> Result<(), Error> {
        let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;

        let packet_delimiter_count = packet_structure.delimiters.len();
        let end_of_packet = packet_structure.size();

        // TODO: check for collisions
        packet_structure
            .delimiters
            .push(PacketDelimiter {
                index: packet_delimiter_count,
                name: format!("Delimiter {}", packet_delimiter_count + 1),
                identifier: vec![0xFF],
                offset_in_packet: end_of_packet,
            });

        return Ok(());
    }

    /// Creates a gap after the specified component_index
    pub fn add_gap_after(
        &mut self,
        packet_structure_id: usize,
        is_field: bool,
        component_index: usize,
    ) -> Result<(), Error> {
        // TODO: Doesn't seem to work when the current index is a gap or the end of a packet
        let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;

        let minimum_offset = if is_field {
            packet_structure.fields[component_index].offset_in_packet
        } else {
            packet_structure.delimiters[component_index].offset_in_packet
        };

        Self::shift_components_after(packet_structure, 1, minimum_offset)?;

        return Ok(());
    }

    ///deletes a component, taking its index and type as parameters
    pub fn delete_packet_structure_component(
        &mut self,
        packet_structure_id: usize,
        component_index: usize,
        component_type: PacketComponentType,
    ) -> Result<(), Error> {
        let packet_structures = self.packet_structures.clone();

        match component_type {
            PacketComponentType::Field => {
                let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;

                if packet_structure.fields.len() == 1 {
                    return Err(Error::CannotDeleteLastField);
                }

                let removed_field = packet_structure.fields.remove(component_index);
                Self::shift_components_after(
                    packet_structure,
                    -(removed_field.r#type.size() as isize),
                    removed_field.offset_in_packet,
                )?;

                for field in &mut packet_structure.fields {
                    if field.index > removed_field.index {
                        field.index -= 1;
                    }
                }
            }
            PacketComponentType::Delimiter => {//also doesn't seem to work
                let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;

                if packet_structure.delimiters.len() == 1 {
                    return Err(Error::CannotDeleteLastDelimiter);
                }

                let removed_delimiter = packet_structure.delimiters.remove(component_index);

                if let Some(colliding_ids) = Self::check_for_identifier_collisions(
                    &packet_structures,
                    packet_structure_id,
                    &packet_structure.delimiters,
                ) {
                    return Err(Error::DelimiterIdentifierCollision(colliding_ids));
                }

                let packet_structure = self.get_packet_structure_mut(packet_structure_id)?;
                Self::shift_components_after(
                    packet_structure,
                    -(removed_delimiter.identifier.len() as isize),
                    removed_delimiter.offset_in_packet,
                )?;
                for delimiter in &mut packet_structure.delimiters {
                    if delimiter.index > removed_delimiter.index {
                        delimiter.index -= 1;
                    }
                }
            }
            PacketComponentType::Gap => {
                // TODO: not working
                Self::set_gap_size(self, packet_structure_id, component_index, 0)?;
            }
        };

        Ok(())
    }

    /// Deletes the packet structure with the given packet structure id
    pub fn delete_packet_structure(&mut self, packet_structure_id: usize) -> Result<(), Error> {
        self.packet_structures.retain(|packet_structure| packet_structure.id != packet_structure_id);
        Ok(())
    }


    /// Updates all of the universal values in the manager 
    /// 
    /// this isn't the most efficient way of doing this since 
    /// not all of these values need to be updated every time the function is called.
    /// choose to write it this way since its more general and can be reused a lot in the code
    /// keep in mind that the packet structure manager does not need to be super fast since its not done while the groundstation is running
    /// 
    /// call this function whenever a packet structures length is changed, or a delimiters location is changed
    fn update_tracked_values(&mut self){
        let mut min_ps = usize::MAX;
        let mut max_ps = 0;
        let mut max_delim = 0;
        for ps in &self.packet_structures {
            let packet_size = ps.size();
            min_ps = min(self.minimum_packet_structure_size, packet_size);
            max_ps = max(self.maximum_packet_structure_size, packet_size);

            // in tests, we can have packets with no delimiters
            match ps.delimiters.get(0) {
                Some(delimiter) => {
                    max_delim = max(self.maximum_first_delimiter, delimiter.offset_in_packet)
                },
                None => {}
            }
        }
        self.minimum_packet_structure_size = min_ps;
        self.maximum_packet_structure_size = max_ps;
        self.maximum_first_delimiter = max_delim;
    }
}






#[cfg(test)]
mod tests {
    use super::*; // lets the unit tests use everything in this file

    #[test]
    fn get_unknown_packet() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let fake_id = 0xdeadbeef;
        assert_eq!(packet_structure_manager.get_packet_structure(fake_id), Err(Error::PacketDoesNotExist(fake_id)));
        assert_eq!(packet_structure_manager.get_packet_structure_mut(fake_id), Err(Error::PacketDoesNotExist(fake_id)));
    }

    #[test]
    fn test_set_packet_name(){
        // create a manager object so we can test its behavior
        let mut packet_structure_manager = PacketStructureManager::default();

        // add a test packet
        let id = packet_structure_manager.register_packet_structure(&mut PacketStructure {
            id: 0, // id is overridden
            name: String::from("First Name"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
        }).unwrap();

        // run the manager function we are trying to test on the test packet
        packet_structure_manager.set_packet_name(id, "Second Name").unwrap();

        // checks that the change we wanted actually happened
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().name, "Second Name");
    }

    // add a unit test that checks for updated minimum and maximum trackers
    #[test]
    fn test_set_field_name() {
        let packet_field_type = PacketFieldType::Double;
        let packet_field = PacketField {
            index: 0,
            name: String::from("not name"),
            r#type: packet_field_type,
            offset_in_packet: 0
        };

        // add our test packet
        let mut packet_structure_manager = PacketStructureManager::default();
        let id = packet_structure_manager.register_packet_structure(&mut PacketStructure {
            id: 0, // gets overridden
            name: String::from("First Name"),
            fields: vec![packet_field],
            delimiters: vec![],
            metafields: vec![],
        }).unwrap();

        packet_structure_manager.set_field_name(id, 0, "name").unwrap();

        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().fields[0].name, "name")
    }

    #[test]
    fn test_set_field_type() {
        let array: [PacketFieldType; 4] = [
            PacketFieldType::UnsignedByte,
            PacketFieldType::SignedShort,
            PacketFieldType::Float,
            PacketFieldType::Double
        ];

        for field_type in array {
            for field_type2 in array {
                let packet_field = PacketField {
                    index: 0,
                    name: String::from("name"),
                    r#type: field_type,
                    offset_in_packet: 0,
                };
                let packet_field2 = PacketField {
                    index: 1,
                    name: String::from("name2"),
                    r#type: field_type,
                    offset_in_packet: field_type.size(),
                };

                // create a test packet
                let mut packet_structure_manager = PacketStructureManager::default();
                let id = packet_structure_manager.register_packet_structure(&mut PacketStructure {
                    id: 0, // gets overridden
                    name: String::from("First Name"),
                    fields: vec![packet_field, packet_field2],
                    delimiters: vec![],
                    metafields: vec![],
                }).unwrap();

                packet_structure_manager.set_field_type(id, 0, field_type2).unwrap();
                assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().fields[0].r#type, field_type2);
                assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().fields[1].offset_in_packet, field_type2.size())
            }
        }
    }

    #[test]
    fn test_set_field_metadata_type() {
        let packet_field_type = PacketFieldType::Double;
        let packet_metadata_type2 = PacketMetadataType::Timestamp;
        let packet_field = PacketField {
            index: 0,
            name: String::from("name"),
            r#type: packet_field_type,
            offset_in_packet: 0,
        };

        let mut packet_structure_manager = PacketStructureManager::default();
        let id = packet_structure_manager.register_packet_structure(&mut PacketStructure {
            id: 0, // gets overridden
            name: String::from("First Name"),
            fields: vec![packet_field],
            delimiters: vec![],
            metafields: vec![],
        }).unwrap();

        packet_structure_manager.set_field_metadata_type(id, 0, packet_metadata_type2).unwrap();
    }

    #[test]
    fn test_set_delimiter_name() {
        let packet_delimiter = PacketDelimiter {
            index: 0,
            name: String::from("delimiter_name"),
            identifier: vec![],
            offset_in_packet: 0
        };

        let mut packet_structure_manager = PacketStructureManager::default(); //initializes a manager object
        let id = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object
            id: 0, // gets overridden
            name: String::from("First Name"),
            fields: vec![],
            delimiters: vec![packet_delimiter],
            metafields: vec![],
        }).unwrap();

        packet_structure_manager.set_delimiter_name(id, 0, "new_name").unwrap();

        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[0].name, "new_name")
    }

    #[test]
    fn test_set_delimiter_identifier() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let id = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object
            id: 0, // ignored
            name: String::from("First Name"),
            fields: vec![],
            delimiters: vec![
                PacketDelimiter {
                    index: 0,
                    name: String::from("Delimiter 1"),
                    identifier: vec![0x12, 0x34],
                    offset_in_packet: 1
                },
                PacketDelimiter {
                    index: 1,
                    name: String::from("Delimiter 2"),
                    identifier: vec![0xab, 0xcd],
                    offset_in_packet: 3
                },
            ],
            metafields: vec![],
        }).unwrap();

        // First we test just changing the identifier to something with the same size
        // Expecting no change in offsets
        packet_structure_manager.set_delimiter_identifier(id, 0, "7856").unwrap();
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[0].identifier, vec![0x78, 0x56]);
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[0].offset_in_packet, 1);
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[1].identifier, vec![0xab, 0xcd]);
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[1].offset_in_packet, 3);

        // Now we try making the delimiter smaller
        // Second delimiter should move forward
        packet_structure_manager.set_delimiter_identifier(id, 0, "f5").unwrap();
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[0].identifier, vec![0xf5]);
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[0].offset_in_packet, 1);
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[1].identifier, vec![0xab, 0xcd]);
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[1].offset_in_packet, 2);

        // Then we try making the delimiter bigger
        // Second delimiter should move backward
        packet_structure_manager.set_delimiter_identifier(id, 0, "0a1b2c3d4e5f6e7d8c9b0a").unwrap();
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[0].identifier, vec![0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x6e, 0x7d, 0x8c, 0x9b, 0x0a]);
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[0].offset_in_packet, 1);
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[1].identifier, vec![0xab, 0xcd]);
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters[1].offset_in_packet, 12);
    }

    #[test]
    fn test_set_gap_size() {
        let mut packet_structure_manager = PacketStructureManager::default();
        let id = packet_structure_manager.register_packet_structure(&mut PacketStructure {
            id: 0, // overwritten
            name: String::from("Test Packet"),
            fields: vec![
                // 1 byte gap at offset 0
                PacketField {
                    index: 0,
                    name: String::from("Field 1"),
                    r#type: PacketFieldType::SignedByte,
                    offset_in_packet: 1
                },
                // 2 byte gap at offset 2
                PacketField {
                    index: 1,
                    name: String::from("Field 2"),
                    r#type: PacketFieldType::SignedByte,
                    offset_in_packet: 4
                }
            ],
            delimiters: vec![
                // 1 byte gap at offset 5
                PacketDelimiter {
                    index: 0,
                    name: String::from("Delimiter 1"),
                    identifier: vec![0x12],
                    offset_in_packet: 6
                },
                // 5 byte gap at offset 7
                PacketDelimiter {
                    index: 1,
                    name: String::from("Delimiter 2"),
                    identifier: vec![0x34],
                    offset_in_packet: 12
                },
            ],
            metafields: vec![],
        }).unwrap();

        // This should move everything back by 1 byte
        packet_structure_manager.set_gap_size(id, 0, 2).unwrap();
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().fields, vec![
            // 2 byte gap at offset 0
            PacketField {
                index: 0,
                name: String::from("Field 1"),
                r#type: PacketFieldType::SignedByte,
                offset_in_packet: 2
            },
            // 2 byte gap at offset 4
            PacketField {
                index: 1,
                name: String::from("Field 2"),
                r#type: PacketFieldType::SignedByte,
                offset_in_packet: 5
            }
        ]);
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters, vec![
            // 1 byte gap at offset 6
            PacketDelimiter {
                index: 0,
                name: String::from("Delimiter 1"),
                identifier: vec![0x12],
                offset_in_packet: 7
            },
            // 5 byte gap at offset 8
            PacketDelimiter {
                index: 1,
                name: String::from("Delimiter 2"),
                identifier: vec![0x34],
                offset_in_packet: 13
            },
        ]);

        // This should move just the last delimiter up by 4 bytes
        packet_structure_manager.set_gap_size(id, 8, 1).unwrap();
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().fields, vec![
            // 2 byte gap at offset 0
            PacketField {
                index: 0,
                name: String::from("Field 1"),
                r#type: PacketFieldType::SignedByte,
                offset_in_packet: 2
            },
            // 2 byte gap at offset 4
            PacketField {
                index: 1,
                name: String::from("Field 2"),
                r#type: PacketFieldType::SignedByte,
                offset_in_packet: 5
            }
        ]);
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters, vec![
            // 1 byte gap at offset 6
            PacketDelimiter {
                index: 0,
                name: String::from("Delimiter 1"),
                identifier: vec![0x12],
                offset_in_packet: 7
            },
            // 1 byte gap at offset 8
            PacketDelimiter {
                index: 1,
                name: String::from("Delimiter 2"),
                identifier: vec![0x34],
                offset_in_packet: 9
            },
        ]);
    }

    #[test]
    fn test_add_field() {
        let mut packet_structure_manager = PacketStructureManager::default(); //initializes a manager object
        let id = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object
            id: 0,
            name: String::from("First Name"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
        }).unwrap();

        packet_structure_manager.add_field(id).unwrap();
        packet_structure_manager.add_field(id).unwrap();
        packet_structure_manager.add_field(id).unwrap();
        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().fields.len(), 3)
    }

    #[test]
    fn test_add_delimiter() {
        let mut packet_structure_manager = PacketStructureManager::default(); //initializes a manager object
        let id = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object
            id: 0,
            name: String::from("First Name"),
            fields: vec![],
            delimiters: vec![],
            metafields: vec![],
        }).unwrap();

        packet_structure_manager.add_delimiter(id).unwrap();
        packet_structure_manager.add_delimiter(id).unwrap();
        packet_structure_manager.add_delimiter(id).unwrap();

        assert_eq!(packet_structure_manager.get_packet_structure(id).unwrap().delimiters.len(), 3)
    }

    // #[test]
    // fn test_add_gap_after() {

    // }

    // #[test]
    // fn test_delete_packet_structure_component() {

    // }

    // #[test]
    // fn test_delete_packet_structure() {

    // }
}
