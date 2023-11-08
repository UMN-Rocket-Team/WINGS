use serde::Serialize;

use crate::models::packet_structure::PacketFieldType;

#[derive(PartialEq, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Represents a packet of data
/// 
/// This includes all of the variables that have been recieved within a packet of data and its timestamp
pub struct Packet {
    pub(crate) structure_id: usize,
    pub(crate) field_data: Vec<PacketFieldValue>,
    pub(crate) timestamp: i64,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
pub enum PacketFieldValue {
    // Ensure that this enum is in sync with PacketFieldType

    #[serde(rename = "Unsigned Byte")]
    UnsignedByte(u8),
    #[serde(rename = "Signed Byte")]
    SignedByte(i8),
    #[serde(rename = "Unsigned Short")]
    UnsignedShort(u16),
    #[serde(rename = "Signed Short")]
    SignedShort(i16),
    #[serde(rename = "Unsigned Integer")]
    UnsignedInteger(u32),
    #[serde(rename = "Signed Integer")]
    SignedInteger(i32),
    #[serde(rename = "Unsigned Long")]
    UnsignedLong(u64),
    #[serde(rename = "Signed Long")]
    SignedLong(i64),
    Float(f32),
    Double(f64),
}

impl PacketFieldType {
    /// takes raw bytes and assigns them the PacketFieldType which they match
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

    /// returns the size of the data included within the packetFieldType
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
