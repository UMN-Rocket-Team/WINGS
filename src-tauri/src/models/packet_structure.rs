use std::cmp::max;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Clone)]
pub struct PacketStructure {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) fields: Vec<PacketField>,
    pub(crate) delimiters: Vec<PacketDelimiter>,
}

impl PacketStructure {
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
pub struct PacketField {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) r#type: PacketFieldType,
    pub(crate) offset_in_packet: usize,
    pub(crate) metadata_type: PacketMetadataType,
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PacketFieldType {
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
pub struct PacketDelimiter {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) identifier: Vec<u8>,
    pub(crate) offset_in_packet: usize,
}
