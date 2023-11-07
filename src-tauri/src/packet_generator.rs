use crate::models::{packet_structure::{PacketStructure, PacketMetadataType, PacketFieldType}, packet::PacketFieldValue};

/// Generate bytes from a packet structure and a list of values.
/// The items in field_data correspond with the `index` property for each field in the packet
/// structure.
/// 
/// # Errors
/// 
/// Some mistakes will be caught and an Err will be returned:
/// 
///  - field.index out-of-bounds
///  - field.index points to a value of a different type
///  - fields marked as timestamp metadata that aren't signed longs
/// 
/// However some errors will not be caught such as packets overlapping each other, so please
/// make sure your structures are valid first :)
pub fn generate_packet(packet_structure: &PacketStructure, field_data: &Vec<PacketFieldValue>) -> Result<Vec<u8>, String> {
    // Calculate size ahead of time so we can make the vec the right size right from the start
    let mut packet_size = 0;
    for delimiter in &packet_structure.delimiters {
        let delimeter_max_index = delimiter.offset_in_packet + delimiter.identifier.len();
        if delimeter_max_index > packet_size {
            packet_size = delimeter_max_index;
        }
    }
    for field in &packet_structure.fields {
        let field_max_index = field.offset_in_packet + field.r#type.size();
        if field_max_index > packet_size {
            packet_size = field_max_index;
        }
    }

    // Don't grow this after making it. It's already the exact right size.
    let mut result = vec![0; packet_size];

    for delimiter in &packet_structure.delimiters {
        let bytes = &delimiter.identifier;
        for i in 0..bytes.len() {
            // guaranteed to not panic due to buffer size calculation previously
            result[delimiter.offset_in_packet + i] = bytes[i];
        }
    }

    for field in &packet_structure.fields {
        let given_value = match field_data.get(field.index) {
            Some(value) => value,
            None => return Err(format!("Field {} refers to missing index: {}", field.name, field.index))
        };

        if field.r#type != given_value.get_field_type() {
            return Err(format!("Field {} has type {:?} but the given value is {:?}", field.name, field.r#type, given_value));
        }

        match (field.metadata_type, field.r#type) {
            (PacketMetadataType::Timestamp, PacketFieldType::SignedLong) => {
                // Only valid type for Timestamp
            },
            (PacketMetadataType::Timestamp, _) => {
                return Err(format!("Field {} is marked as timestamp but is type {:?}", field.name, field.r#type));
            },
            (_, _) => {
                // No other illegal combinations
            }
        }

        let bytes = given_value.to_le_bytes();
        for i in 0..bytes.len() {
            // guaranteed to not panic due to buffer size calculation previously
            result[field.offset_in_packet + i] = bytes[i];
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::models::{packet_structure::{PacketStructure, PacketDelimiter, PacketField, PacketMetadataType, PacketFieldType}, packet::PacketFieldValue};

    use super::generate_packet;

    #[test]
    fn empty_packet() {
        let structure = PacketStructure {
            id: 0,
            name: "Empty Packet".to_string(),
            delimiters: vec![],
            fields: vec![]
        };
        let packet = generate_packet(&structure, &vec![]).unwrap();
        assert_eq!(packet.len(), 0);
    }

    #[test]
    fn delimiter() {
        let structure = PacketStructure {
            id: 0,
            name: "Test Packet".to_string(),
            delimiters: vec![
                PacketDelimiter {
                    index: 0,
                    name: "Test Delimter".to_string(),
                    offset_in_packet: 1,
                    identifier: vec![42, 43, 45]
                }
            ],
            fields: vec![]
        };
        let packet = generate_packet(&structure, &vec![]).unwrap();
        assert_eq!(packet, [0, 42, 43, 45]);
    }

    #[test]
    fn field() {
        let structure = PacketStructure {
            id: 0,
            name: "Test Packet".to_string(),
            delimiters: vec![],
            fields: vec![
                PacketField {
                    index: 0,
                    name: "Test Field".to_string(),
                    offset_in_packet: 5,
                    metadata_type: PacketMetadataType::None,
                    r#type: PacketFieldType::UnsignedInteger
                }
            ]
        };
        let packet = generate_packet(&structure, &vec![
            PacketFieldValue::UnsignedInteger(0x12345678)
        ]).unwrap();
        assert_eq!(packet, [
            // padding bytes
            0, 0, 0, 0, 0,
            // then the field value in little-endian
            0x78, 0x56, 0x34, 0x12
        ]);
    }

    #[test]
    fn field_index_out_of_bounds() {
        let structure = PacketStructure {
            id: 0,
            name: "Test Packet".to_string(),
            delimiters: vec![],
            fields: vec![
                PacketField {
                    index: 0,
                    name: "Test Field".to_string(),
                    offset_in_packet: 0,
                    metadata_type: PacketMetadataType::None,
                    r#type: PacketFieldType::UnsignedByte
                }
            ]
        };
        // notice that we provide no packet values when we should provide some
        let packet = generate_packet(&structure, &vec![]);
        assert_eq!(packet.unwrap_err(), "Field Test Field refers to missing index: 0");
    }

    #[test]
    fn field_type_and_value_mismatch() {
        let structure = PacketStructure {
            id: 0,
            name: "Test Packet".to_string(),
            delimiters: vec![],
            fields: vec![
                PacketField {
                    index: 0,
                    name: "Test Field".to_string(),
                    offset_in_packet: 0,
                    metadata_type: PacketMetadataType::None,
                    r#type: PacketFieldType::UnsignedByte
                }
            ]
        };
        let packet = generate_packet(&structure, &vec![
            // SignedByte != UnsignedByte
            PacketFieldValue::SignedByte(16)
        ]);
        assert_eq!(packet.unwrap_err(), "Field Test Field has type UnsignedByte but the given value is SignedByte(16)");
    }

    #[test]
    fn timestamp_metadata() {
        let bad_structure = PacketStructure {
            id: 0,
            name: "Test Packet 1".to_string(),
            delimiters: vec![],
            fields: vec![
                PacketField {
                    index: 0,
                    name: "Test Field 1".to_string(),
                    offset_in_packet: 0,
                    metadata_type: PacketMetadataType::Timestamp,
                    r#type: PacketFieldType::SignedByte
                }
            ]
        };
        let bad_packet = generate_packet(&bad_structure, &vec![
            PacketFieldValue::SignedByte(101)
        ]);
        assert_eq!(bad_packet.unwrap_err(), "Field Test Field 1 is marked as timestamp but is type SignedByte");

        let good_structure = PacketStructure {
            id: 0,
            name: "Test Packet 2".to_string(),
            delimiters: vec![],
            fields: vec![
                PacketField {
                    index: 0,
                    name: "Test Field 2".to_string(),
                    offset_in_packet: 0,
                    metadata_type: PacketMetadataType::Timestamp,
                    r#type: PacketFieldType::SignedLong
                }
            ]
        };
        let good_packet = generate_packet(&good_structure, &vec![
            PacketFieldValue::SignedLong(1699481341632)
        ]);
        assert_eq!(good_packet.unwrap(), u64::to_le_bytes(1699481341632));
    }

    #[test]
    fn fields_and_delimiters() {
        let structure = PacketStructure {
            id: 100000, // should be unused ...
            name: "Test Packet".to_string(),
            delimiters: vec![
                PacketDelimiter {
                    index: 0,
                    name: "Test Delimiter".to_string(),
                    offset_in_packet: 0,
                    identifier: vec![1, 2, 3]
                },
                PacketDelimiter {
                    index: 200, // should be ignored
                    name: "Test Delimiter 2".to_string(),
                    offset_in_packet: 15,
                    identifier: vec![4, 5, 6]
                }
            ],
            fields: vec![
                PacketField {
                    index: 0,
                    name: "Test Field 1".to_string(),
                    offset_in_packet: 3,
                    metadata_type: PacketMetadataType::None,
                    r#type: PacketFieldType::Float
                },
                PacketField {
                    index: 2,
                    name: "Test Field 2".to_string(),
                    offset_in_packet: 7,
                    metadata_type: PacketMetadataType::Timestamp,
                    r#type: PacketFieldType::SignedLong
                }
            ]
        };
        let packet = generate_packet(&structure, &vec![
            PacketFieldValue::Float(3.0),
            PacketFieldValue::Double(6.0), // should be unused
            PacketFieldValue::SignedLong(0x1234),
        ]).unwrap();
        assert_eq!(packet, [
            // Delimiter 1
            1, 2, 3,
            // Test Field 1 value, little-endian
            0, 0, 64, 64,
            // Test Field 2 value, little-endian
            0x34, 0x12, 0, 0, 0, 0, 0, 0,
            // Delimiter 2
            4, 5, 6
        ]);
    }
}
