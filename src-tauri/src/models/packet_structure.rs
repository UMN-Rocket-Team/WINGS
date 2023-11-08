use std::cmp::max;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Clone)]

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
