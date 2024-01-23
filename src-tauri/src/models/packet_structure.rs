use std::cmp::max;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Clone, Debug)]

/// Represents an entire "Data Packet Structure" 
/// 
/// This is the data packet format in which the ground station should expect to recieve new data
pub struct PacketStructure {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) fields: Vec<PacketField>,
    pub(crate) delimiters: Vec<PacketDelimiter>,
}

impl PacketStructure {

    /// Returns the size of the PacketStructure
    /// 
    /// size is found by going through every element in the packetstructure, and finding the largest sum of an elements offset and size.
    /// this produces the same result as returning the largest offset with the size of that element added on
    pub fn size(&self) -> usize {
        let mut max_end: usize = 0;

        for field in &self.fields {
            max_end = max(max_end, field.offset_in_packet + field.r#type.size());
        }

        for delimiter in &self.delimiters {
            max_end = max(
                max_end,
                delimiter.offset_in_packet + delimiter.identifier.len(),
            );
        }

        max_end
    }

    /// fills the packet calling it using string inputs,
    /// THIS IS MENT FOR TESTING ONLY
    /// 
    /// 0 - f,    represents delimiters in hex
    /// _1-inf    represents gaps, the number after is the length in decimal
    /// u8 - u64  represents unsigned ints
    /// i8 - i64  represents signed ints
    /// F32 & F64 represents floats
    /// 
    /// all delimiters will be named "test delimiter" and all fields "test field"
    /// 
    /// spaces are used to format between elements
    /// ie "deadbeef _4 u8 u8 i16 i16 deadbeef" is 2 delimiters and 4 variables and a 4byte gap
    pub fn ez_make(&mut self, input: &str) {
        let mut curr_offset = 0;
        for substr in input.split(" ") {
            let first_char = substr.chars().nth(0).unwrap();
            if first_char.is_digit(16){
                let mut new_identifier = hex::decode(substr).unwrap();
                new_identifier.reverse();

                let new_delimiter = PacketDelimiter{
                    index: self.delimiters.len(),
                    name:"test delimiter".to_string(),
                    identifier: new_identifier,
                    offset_in_packet: curr_offset
                };
                curr_offset += new_delimiter.identifier.len();
                self.delimiters.push(new_delimiter);
            }
            else if first_char == '_' {
                let trimmedstr = substr.chars().next().map(|c| &substr[c.len_utf8()..]);
                curr_offset += trimmedstr.unwrap().parse::<usize>().unwrap();
            }
            else{
                let offset: usize;
                let t: PacketFieldType;
                match substr{
                        "u8" => {offset = 1; t = PacketFieldType::UnsignedByte},
                        "i8" => {offset = 1; t = PacketFieldType::SignedByte},
                        "u16" => {offset = 2; t = PacketFieldType::UnsignedShort},
                        "i16" => {offset = 2; t = PacketFieldType::SignedShort},
                        "u32" => {offset = 4; t = PacketFieldType::UnsignedInteger},
                        "i32" => {offset = 4; t = PacketFieldType::SignedInteger},
                        "u64" => {offset = 8; t = PacketFieldType::UnsignedLong},
                        "i64" => {offset = 8; t = PacketFieldType::SignedLong},
                        "F32" => {offset = 4; t = PacketFieldType::Float},
                        "F64" => {offset = 8; t = PacketFieldType::Double},
                        &_ => {offset = 0; t = PacketFieldType::UnsignedByte},
                }
                let new_field = PacketField{
                    index: self.fields.len(),
                    name:"test field".to_string(),
                    r#type: t,
                    offset_in_packet: curr_offset,
                    metadata_type: PacketMetadataType::None,
                };
                self.fields.push(new_field);
                curr_offset += offset;

            }
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]

/// represents a field within a Packet where the groundstation can expect a piece of data to be stored.
/// 
/// each packet field contains a number based datatype, 
/// a Packetfield can also potentially contain timestamp metadata
pub struct PacketField {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) r#type: PacketFieldType,
    pub(crate) offset_in_packet: usize,
    pub(crate) metadata_type: PacketMetadataType,
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
///Represents the different types that can be recieved in a packetField
pub enum PacketFieldType {
    // Ensure that this enum is in sync with PacketFieldValue

    #[serde(rename = "Unsigned Byte")]
    UnsignedByte,
    #[serde(rename = "Signed Byte")]
    SignedByte,
    #[serde(rename = "Unsigned Short")]
    UnsignedShort,
    #[serde(rename = "Signed Short")]
    SignedShort,
    #[serde(rename = "Unsigned Integer")]
    UnsignedInteger,
    #[serde(rename = "Signed Integer")]
    SignedInteger,
    #[serde(rename = "Unsigned Long")]
    UnsignedLong,
    #[serde(rename = "Signed Long")]
    SignedLong,
    Float,
    Double,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PacketMetadataType {
    None,
    Timestamp,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
///Represents a Delimiter within a Packet that can be used to identify that packet within the raw data that is recieved by radio
/// 
///The identifier variable represents the unique set of data that the groundstation should look out for when looking at the incoming datastream
pub struct PacketDelimiter {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) identifier: Vec<u8>,
    pub(crate) offset_in_packet: usize,
}
