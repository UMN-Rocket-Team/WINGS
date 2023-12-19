use std::{
    cmp::{max, min},
    vec,
};




use crate::{
    models::packet_structure::{
        PacketDelimiter, PacketField, PacketFieldType, PacketMetadataType, PacketStructure,
    },
    models::packet_view_model::PacketComponentType,
};

//A packet structure manager is an object that contains all the packets the app is dealing with, this makes them easier to use them from other threads and handle errors
#[readonly::make]
pub struct PacketStructureManager {
    pub(crate) packet_structures: Vec<PacketStructure>, 
    pub(crate) minimum_packet_structure_size: usize, // These variables are used in situations when matching packets to bits
    pub(crate) maximum_packet_structure_size: usize,
    pub(crate) maximum_first_delimiter: usize,
}

impl Default for PacketStructureManager {
    fn default() -> Self {
        Self {
            packet_structures: Default::default(),
            minimum_packet_structure_size: usize::MAX,
            maximum_packet_structure_size: 0,
            maximum_first_delimiter: 0,
        }
    }
}

#[derive(Debug)]
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

pub enum DeletePacketStructureComponentError {
    LastField,
    LastDelimiter,
    DelimiterIdentifierCollision(Vec<usize>),
}

impl PacketStructureManager {

    ///takes the given PacketStructure and makes a copy within the manager for future use
    ///Note: this also needs to be unit tested
    pub fn register_packet_structure(
        &mut self,
        packet_structure: &mut PacketStructure,
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

        packet_structure.id = self.packet_structures.len();

        self.packet_structures.push(packet_structure.clone());
        self.update_tracked_values();
        Ok(packet_structure.id)
    }

    ///sets a packet structures name, uses the id in order to find the packet structure within the 
    pub fn set_packet_name(&mut self, packet_structure_id: usize, name: &str) {
        self.packet_structures[packet_structure_id].name = String::from(name);
    }
    ///sets the name of a specific field within a specific packet structure
    ///packet_structure_id is used to find the packet_structure within the manager
    ///field_index is used to find the field within the packetstructure
    pub fn set_field_name(&mut self, packet_structure_id: usize, field_index: usize, name: &str) {
        self.packet_structures[packet_structure_id].fields[field_index].name = String::from(name);
    }

    ///shifts components all componets after a specified point in the packet structure
    ///ex:
    ///before:
    ///[][][][][][][]
    ///after:
    ///[][][]  [][][][]
    ///
    ///offset_diff is the size we want to shift by
    ///minimum_offset is where the shifting will start
    fn shift_components_after(
        packet_structure: &mut PacketStructure,
        offset_diff: isize,
        minimum_offset: usize,
    ) {
        for field in &mut packet_structure.fields {
            if field.offset_in_packet > minimum_offset {
                field.offset_in_packet = field
                    .offset_in_packet
                    .checked_add_signed(offset_diff)//checking for overflow
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
    /// sets the data type we can expect for a field, and adjusts all following packets to make room for the new data
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
        self.update_tracked_values();
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

    /// Adapted from https://codereview.stackexchange.com/a/201699
    fn get_hex_char_value(hex_char: u8) -> Option<u8> {
        match hex_char {
            b'0'..=b'9' => Some(hex_char - b'0'),
            b'a'..=b'f' => Some(hex_char - b'a' + 10),
            b'A'..=b'F' => Some(hex_char - b'A' + 10),
            _ => None,
        }
    }

    ///takes a hexadeximal string and returns its binary equivilant
    ///(currently used primarly to decode the user input for delimiter identifiers)
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

    ///checks if a packet structure in packets structures(Specified by id) has the same delimiters as another packet structure in packet structures
    fn check_for_identifier_collisions(
        packet_structures: &Vec<PacketStructure>,
        packet_structure_id: usize,
        delimiters: &Vec<PacketDelimiter>,
    ) -> Result<(), Vec<usize>> {
        let mut identifier_collisions = Vec::new();

        for other_packet_structure in packet_structures {
            if other_packet_structure.id == packet_structure_id {
                continue;
            }

            if other_packet_structure.delimiters == *delimiters {// this feels like it could miss some problematic delimiters(make sure to unit test extensively)
                identifier_collisions.push(other_packet_structure.id);
            }
        }

        if !identifier_collisions.is_empty() {
            return Err(identifier_collisions);
        }
        Ok(())
    }

    ///takes a delimiter and changes its identifier, it then shifts all other packets as neccisary to keep offset relative to new identifier
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
        if let Err(colliding_ids) = Self::check_for_identifier_collisions(
            &self.packet_structures,
            packet_structure_id,
            &delimiters,
        ) {
            return Err(SetDelimiterIdentifierError::IdentifierCollision(
                colliding_ids,
            ));
        }

        // Apply the change
        let packet_structure = &mut self.packet_structures[packet_structure_id];
        packet_structure.delimiters = delimiters;

        Self::shift_components_after(packet_structure, offset_diff, delimiter_offset);
        self.update_tracked_values();
        Ok(())
    }

    ///looks for a field or delimiter with the given offset
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

    ///changes the size of a gap between two items inside a packet
    /// gap_index is the marking of the gaps location in the packet based off of the number of gaps before it(nth gap)
    /// gap_size is the size we want to set the gap to
    /// should probably be rewritten to use gap offset rather than index
    pub fn set_gap_size(&mut self, packet_structure_id: usize, gap_index: usize, gap_size: usize) {
        // Find the gap with the given index
        let mut found_gap_index: usize = 0;

        let mut previous_field_or_delimiter: Option<PacketFieldOrDelimiter> = None;
        let mut maybe_field_or_delimiter =
            self.find_field_or_delimiter_with_offset(packet_structure_id, 0);

        // itterates and finds every gap in the packet until it reaches the nth gap(gap_index), it then shifts everything after the gap
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
        self.update_tracked_values();
    }

    ///creates a new field inside the end of the packet
    //needs to be reworked to add to a user-specified location
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
        self.update_tracked_values();
    }
    ///creates a new delimeter inside the end of the packet
    //needs to be reworked to add to a user-specified location
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
        self.update_tracked_values();
    }

    /// Creates a gap after the specified component_index
    /// 
    /// Doesnt work when the current index is a gap
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
        self.update_tracked_values();
    }

    ///deletes a component, taking its index and type as parameters
    pub fn delete_packet_structure_component(//probably could be reworked to not take the type(maybe split this into three diferent functions)
        &mut self,
        packet_structure_id: usize,
        component_index: usize,
        component_type: PacketComponentType,
    ) -> Result<(), DeletePacketStructureComponentError> {
        match component_type {
            PacketComponentType::Field => {
                let packet_structure = &mut self.packet_structures[packet_structure_id];

                if packet_structure.fields.len() == 1 {
                    return Err(DeletePacketStructureComponentError::LastField);
                }

                let removed_field = packet_structure.fields.remove(component_index);
                Self::shift_components_after(
                    packet_structure,
                    -(removed_field.r#type.size() as isize),
                    removed_field.offset_in_packet,
                );
                for field in &mut packet_structure.fields {
                    if field.index > removed_field.index {
                        field.index -= 1;
                    }
                }
            }
            PacketComponentType::Delimiter => {//also doesn't seem to work
                let packet_structures = &mut self.packet_structures;

                if packet_structures[packet_structure_id].delimiters.len() == 1 {
                    return Err(DeletePacketStructureComponentError::LastDelimiter);
                }

                let removed_delimiter = packet_structures[packet_structure_id].delimiters.remove(component_index);

                if let Err(colliding_ids) = Self::check_for_identifier_collisions(
                    packet_structures,
                    packet_structure_id,
                    &packet_structures[packet_structure_id].delimiters,
                ) {
                    return Err(
                        DeletePacketStructureComponentError::DelimiterIdentifierCollision(
                            colliding_ids,
                        ),
                    );
                }

                let packet_structure = &mut self.packet_structures[packet_structure_id];

                Self::shift_components_after(
                    packet_structure,
                    -(removed_delimiter.identifier.len() as isize),
                    removed_delimiter.offset_in_packet,
                );
                for delimiter in &mut packet_structure.delimiters {
                    if delimiter.index > removed_delimiter.index {
                        delimiter.index -= 1;
                    }
                }
            }
            PacketComponentType::Gap => {
                Self::set_gap_size(self, packet_structure_id, component_index, 0)//not working
            }
        }
        self.update_tracked_values();
        Ok(())
    }

    ///Deletes the packet structure with the given packet structure id
    pub fn delete_packet_structure(&mut self, packet_structure_id: usize){
        self.packet_structures.retain(|packet_structure| packet_structure.id != packet_structure_id);
        self.update_tracked_values();
    }

    /// Updates all of the universal values in the manager 
    /// 
    /// this isnt the most efficient way of doing this since 
    /// not all of these values need to be updated every time the function is called.
    /// choose to write it this way since its more general and can be reused alot in the code
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
            max_delim = max(self.maximum_first_delimiter,ps.delimiters[0].offset_in_packet);
        }
        self.minimum_packet_structure_size = min_ps;
        self.maximum_packet_structure_size = max_ps;
        self.maximum_first_delimiter = max_delim;
    }
}

#[cfg(test)]
mod tests {
    use super::*;//lets the unit tests use everything in this file
    #[test]
    fn test_set_packet_name(){
        let mut packet_structure_manager = PacketStructureManager::default(); //initializes a manager object
        let _ = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object 
            id: 0,
            name: String::from("First Name"),
            fields: vec![],
            delimiters: vec![],
        });

        packet_structure_manager.set_packet_name(0, "Second Name"); //calls the function we are testing on the manager

        assert_eq!(packet_structure_manager.packet_structures[0].name, "Second Name"); //checks that the change we wanted actually happened
    }
    //add a unit test that checks for updated minimum and maximum trackers
    #[test]
    fn test_set_field_name() {
        let packet_field_type = PacketFieldType::Double;
        let packet_metadata_type = PacketMetadataType::None;
        let packet_field = PacketField{index: 0, name: String::from("notname"), r#type: packet_field_type, offset_in_packet: 0, metadata_type: packet_metadata_type};
        let mut packet_structure_manager = PacketStructureManager::default(); //initializes a manager object
        let _ = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object 
            id: 0,
            name: String::from("First Name"),
            fields: vec![packet_field],
            delimiters: vec![],
        });
        
        packet_structure_manager.set_field_name(0, 0, "name");
      

        assert_eq!(packet_structure_manager.packet_structures[0].fields[0].name, "name")
        
    }

    #[test]
    
    fn test_set_field_type() {
        let packet_field_test_1 = PacketFieldType::UnsignedByte;
        let packet_field_test_2 = PacketFieldType::SignedShort;
        let packet_field_test_3 = PacketFieldType::Float;
        let packet_field_test_4 = PacketFieldType::Double;

        let array: [PacketFieldType;4] = [packet_field_test_1, packet_field_test_2, packet_field_test_3, packet_field_test_4];
        for field_type in array {
            for field_type2 in array {
                let packet_metadata_type = PacketMetadataType::None;
                let packet_field = PacketField{index: 0, name: String::from("name"), r#type: field_type, offset_in_packet: 0, metadata_type: packet_metadata_type};
                let packet_field2 = PacketField{index: 1, name: String::from("name2"), r#type: field_type, offset_in_packet: field_type.size(), metadata_type: packet_metadata_type};
                let mut packet_structure_manager = PacketStructureManager::default(); //initializes a manager object
                let _ = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object 
                    id: 0,
                    name: String::from("First Name"),
                    fields: vec![packet_field, packet_field2],
                    delimiters: vec![],
                });
                packet_structure_manager.set_field_type(0, 0, field_type2);
                assert_eq!(packet_structure_manager.packet_structures[0].fields[0].r#type, field_type2);
                assert_eq!(packet_structure_manager.packet_structures[0].fields[1].offset_in_packet, field_type2.size())

            }
            
        }

    }

    #[test]
    fn test_set_field_metadata_type() {
        let packet_field_type = PacketFieldType::Double;
        let packet_metadata_type = PacketMetadataType::None;
        let packet_metadata_type2 = PacketMetadataType::Timestamp;
        let packet_field = PacketField{index: 0, name: String::from("name"), r#type: packet_field_type, offset_in_packet: 0, metadata_type: packet_metadata_type};
        let mut packet_structure_manager = PacketStructureManager::default(); //initializes a manager object
        let _ = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object 
            id: 0,
            name: String::from("First Name"),
            fields: vec![packet_field],
            delimiters: vec![],
        });

        packet_structure_manager.set_field_metadata_type(0, 0, packet_metadata_type2);

        assert_eq!(packet_structure_manager.packet_structures[0].fields[0].metadata_type, packet_metadata_type2);
    }

    #[test]

    fn test_set_delimiter_name() {
        let packet_delimiter = PacketDelimiter{index: 0, name: String::from("delimiter_name"), identifier: vec![], offset_in_packet: 0};

        let mut packet_structure_manager = PacketStructureManager::default(); //initializes a manager object
        let _ = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object 
            id: 0,
            name: String::from("First Name"),
            fields: vec![],
            delimiters: vec![packet_delimiter],
        });

        packet_structure_manager.set_delimiter_name(0, 0, "new_name");

        assert_eq!(packet_structure_manager.packet_structures[0].delimiters[0].name, "new_name")
    }

    #[test]

    fn test_set_delimiter_identifier() {
        let packet_delimiter = PacketDelimiter{index: 0, name: String::from("delimiter_name"), identifier: vec![1,2], offset_in_packet: 0};
        let packet_delimiter2 = PacketDelimiter{index: 1, name: String::from("delimiter_name"), identifier: vec![1], offset_in_packet: 2};



        let mut packet_structure_manager = PacketStructureManager::default(); //initializes a manager object
        let _ = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object 
            id: 0,
            name: String::from("First Name"),
            fields: vec![],
            delimiters: vec![packet_delimiter, packet_delimiter2],
        });

        
        let _ = packet_structure_manager.set_delimiter_identifier(0, 0, "1");
        
        assert_eq!(packet_structure_manager.packet_structures[0].delimiters[0].identifier, vec![16]);
        assert_eq!(packet_structure_manager.packet_structures[0].delimiters[1].offset_in_packet, vec![16].len());
    }
    
    #[test]

    fn test_set_gap_size() {

    }

    #[test]
    
    fn test_add_field() {
        let mut packet_structure_manager = PacketStructureManager::default(); //initializes a manager object
        let _ = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object 
            id: 0,
            name: String::from("First Name"),
            fields: vec![],
            delimiters: vec![],
        });

        packet_structure_manager.add_field(0);
        packet_structure_manager.add_field(0);
        packet_structure_manager.add_field(0);
        assert_eq!(packet_structure_manager.packet_structures[0].fields.len(), 3)
    }

    #[test]

    fn test_add_delimiter() {
        let mut packet_structure_manager = PacketStructureManager::default(); //initializes a manager object
        let _ = packet_structure_manager.register_packet_structure(&mut PacketStructure { //inserts empty packet into the manager object 
            id: 0,
            name: String::from("First Name"),
            fields: vec![],
            delimiters: vec![],
        });

        packet_structure_manager.add_delimiter(0);
        packet_structure_manager.add_delimiter(0);
        packet_structure_manager.add_delimiter(0);

        assert_eq!(packet_structure_manager.packet_structures[0].delimiters.len(), 3)

    }

    #[test]

    fn test_add_gap_after() {

    }

    #[test]
    
    fn test_delete_packet_structure_component() {

    }

    #[test]

    fn test_delete_packet_structure() {

    }


}
