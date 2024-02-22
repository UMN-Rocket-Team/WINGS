use csv::StringRecord;


use crate::models::{packet::PacketFieldValue, packet_structure::PacketStructure};

/// Generate bytes from a packet structure and a list of values.
///
/// The items in field_data are converted to little endian bytes. Only the needed
/// bytes will actually be used. For example, 0x12345678 becomes LE-bytes:
///     [0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00]]
/// if used in a signed or unsigned short field, only [0x78, 0x56] are used.
/// 
/// # Errors
/// 
/// Some mistakes will be caught and an Err will be returned:
/// 
///  - field.index out-of-bounds
/// 
/// However some errors will not be caught such as packets overlapping each other, so please
/// make sure your structures are valid first :)
pub fn generate_packet(packet_structure: &PacketStructure, field_data: StringRecord) -> Result<Vec<u8>, String> {
    // Don't grow this after making it. It's already the exact right size.
    let mut result = vec![0; packet_structure.size()];

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
        let parsed_value: PacketFieldValue = match field.r#type.make_from_string(given_value){
            Ok(value) => value,
            Err(_)=> return Err(format!("Field {} refers to missing index: {}", field.name, field.index)),
        };
        let bytes = parsed_value.to_le_bytes();
        for i in 0..field.r#type.size() {
            // guaranteed to not panic due to buffer size calculation previously
            result[field.offset_in_packet + i] = bytes[i];
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use csv::StringRecord;

    use crate::models::packet_structure::{PacketDelimiter, PacketField, PacketFieldType, PacketStructure};

    use super::generate_packet;

    #[test]
    fn empty_packet() {
        let structure = PacketStructure {
            id: 0,
            name: "Empty Packet".to_string(),
            delimiters: vec![],
            fields: vec![],
            metafields: vec![],
        };
        let packet = generate_packet(&structure, StringRecord::from(vec![""])).unwrap();
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
            fields: vec![],
            metafields: vec![],
        };
        let packet = generate_packet(&structure, StringRecord::from(vec![""])).unwrap();
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
                    r#type: PacketFieldType::UnsignedInteger
                }
            ],
            metafields: vec![],
        };
        let packet = generate_packet(&structure, StringRecord::from(vec!["305419896"])).unwrap();
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
                    r#type: PacketFieldType::UnsignedByte
                }
            ],
            metafields: vec![],
        };
        // notice that we provide no packet values when we should provide some
        let packet = generate_packet(&structure, StringRecord::from(vec![""]));
        assert_eq!(packet.unwrap_err(), "Field Test Field refers to missing index: 0");
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
                    r#type: PacketFieldType::Float
                },
                PacketField {
                    index: 2,
                    name: "Test Field 2".to_string(),
                    offset_in_packet: 7,
                    r#type: PacketFieldType::SignedLong
                }
            ],
            metafields: vec![],
        };

        let packet = generate_packet(&structure, StringRecord::from(vec![
            "3.0",
            "6.0",
            "-4660"
        ])).unwrap();
        assert_eq!(packet, [
            // Delimiter 1
            1, 2, 3,
            // Test Field 1 value, little-endian
            0, 0, 64, 64,
            // Test Field 2 value, little-endian. Verified using this C program in an Intel computer:
            // #include <stdio.h>
            // int main () {
            //     long test = -0x1234;
            //     unsigned char *ptr = (void*) &test;
            //     for (int i = 0; i < sizeof(test); i++) {
            //         printf("%d - %d\n", i, ptr[i]);
            //     }
            // }
            204, 237, 255, 255, 255, 255, 255, 255,
            // Delimiter 2
            4, 5, 6
        ]);
    }
}
