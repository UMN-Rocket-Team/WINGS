use serde::Serialize;

use crate::models::packet_structure::PacketFieldType;

#[derive(PartialEq, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Represents a packet of data
/// 
/// This includes all of the variables that have been received within a packet of data and its timestamp
pub struct Packet {
    pub(crate) structure_id: usize,
    pub(crate) field_data: Vec<PacketFieldValue>,
}

impl Packet{
    pub fn default(type_id: usize, values: Vec<PacketFieldValue>) -> Packet{
        Packet{
            structure_id: type_id,
            field_data: values
        }
    }
}
#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum PacketFieldValue {
    /// Ensure that this enum is in sync with PacketFieldType
    String(String),
    Bool(bool),
    Number(f64)
}

#[allow(dead_code)]
impl PacketFieldValue {
    pub fn edit_number(&mut self, callback: &mut dyn FnMut(&mut f64) -> f64) {
        match self {
            PacketFieldValue::Number(i) => {*i = callback(i)},
            _ => {},
        }
    }
    pub fn new_number(&mut self, callback: &mut dyn FnMut(&mut f64) -> f64)-> PacketFieldValue {
        let mut new_number = self.clone();
        match &mut new_number {
            PacketFieldValue::Number(i) => {*i = callback(i)},
            _ => {},
        }
        return new_number
    }
    /// Converts this value to a vec of bytes in little-endian form (see CSCI 2021)
    pub fn to_le_bytes(&self,field_type: PacketFieldType) -> anyhow::Result<Vec<u8>> {
        // Need to return a vec here instead of a [u8] as the size is not constant
        match self {
            PacketFieldValue::String(i) => return Ok(i.as_bytes().to_vec()),
            PacketFieldValue::Bool(i) => return Ok(vec![*i as u8]),
            PacketFieldValue::Number(i) => {
                match field_type{
                    PacketFieldType::UnsignedByte => Ok(u8::to_le_bytes(*i as u8).to_vec()),
                    PacketFieldType::SignedByte => Ok(i8::to_le_bytes(*i as i8).to_vec()),
                    PacketFieldType::UnsignedShort => Ok(u16::to_le_bytes(*i as u16).to_vec()),
                    PacketFieldType::SignedShort => Ok(i16::to_le_bytes(*i as i16).to_vec()),
                    PacketFieldType::UnsignedInteger => Ok(u32::to_le_bytes(*i as u32).to_vec()),
                    PacketFieldType::SignedInteger => Ok(i32::to_le_bytes(*i as i32).to_vec()),
                    PacketFieldType::UnsignedLong => Ok(u64::to_le_bytes(*i as u64).to_vec()),
                    PacketFieldType::SignedLong => Ok(i64::to_le_bytes(*i as i64).to_vec()),
                    PacketFieldType::Float => Ok(f32::to_le_bytes(*i as f32).to_vec()),
                    PacketFieldType::Double => Ok(f64::to_le_bytes(*i as f64).to_vec()),
                    PacketFieldType::UnsignedTwoFour => {
                        u32::to_le_bytes(*i as u32).to_vec();
                        todo!("make these only spit out 3 u8s");
                    },
                    PacketFieldType::SignedTwoFour => Ok(i32::to_le_bytes(*i as i32).to_vec()),
                    _ => Err(anyhow::anyhow!("Numbervalue being processed as String or bool"))
                }
            },
        }
    }

    /// Returns the matching PacketFieldType for this parsed value.
    pub fn get_field_type(&self) -> PacketFieldType {
        match self {
            PacketFieldValue::String(_) => PacketFieldType::String,
            PacketFieldValue::Bool(_) => PacketFieldType::Bool,
            PacketFieldValue::Number(_) => PacketFieldType::Double,
        }
    }
    
}

impl PacketFieldType {
    /// takes raw bytes and assigns them the PacketFieldType which they match
    pub fn parse(&self, bytes: &[u8]) -> anyhow::Result<PacketFieldValue> {
        match self {
            PacketFieldType::UnsignedByte => {
                                Ok(PacketFieldValue::Number(u8::from_le_bytes(slice_to_fixed_size::<1>(bytes)).into()))
                            }
            PacketFieldType::SignedByte => {
                                Ok(PacketFieldValue::Number(i8::from_le_bytes(slice_to_fixed_size::<1>(bytes)).into()))
                            }
            PacketFieldType::UnsignedShort => {
                                Ok(PacketFieldValue::Number(u16::from_le_bytes(slice_to_fixed_size::<2>(bytes)).into()))
                            }
            PacketFieldType::SignedShort => {
                                Ok(PacketFieldValue::Number(i16::from_le_bytes(slice_to_fixed_size::<2>(bytes)).into()))
                            }
            PacketFieldType::UnsignedInteger => {
                                Ok(PacketFieldValue::Number(u32::from_le_bytes(slice_to_fixed_size::<4>(bytes)).into()))
                            }
            PacketFieldType::SignedInteger => {
                                Ok(PacketFieldValue::Number(i32::from_le_bytes(slice_to_fixed_size::<4>(bytes)).into()))
                            }
            PacketFieldType::UnsignedLong => {
                                Ok(PacketFieldValue::Number(u64::from_le_bytes(slice_to_fixed_size::<8>(bytes)) as f64))
                            }
            PacketFieldType::SignedLong => {
                                Ok(PacketFieldValue::Number(i64::from_le_bytes(slice_to_fixed_size::<8>(bytes)) as f64))
                            }
            PacketFieldType::Float => {
                                Ok(PacketFieldValue::Number(f32::from_le_bytes(slice_to_fixed_size::<4>(bytes)).into()))
                            }
            PacketFieldType::Double => {
                                Ok(PacketFieldValue::Number(f64::from_le_bytes(slice_to_fixed_size::<8>(bytes)).into()))
                            }
            PacketFieldType::UnsignedTwoFour => {
                        let mut raw:u32 = 0;
                        raw |= (bytes[0] as u32) << 16;
                        raw |= (bytes[1] as u32) << 8;
                        raw |= bytes[2] as u32;
                        Ok(PacketFieldValue::Number(raw.into()))
                    },
            PacketFieldType::SignedTwoFour => {
                        let mut raw:u32 = 0;
                        raw |= (bytes[0] as u32) << 16;
                        raw |= (bytes[1] as u32) << 8;
                        raw |= bytes[2] as u32;
                        Ok(PacketFieldValue::Number(raw.into()))
                    },
            PacketFieldType::String => Ok(PacketFieldValue::String(String::from_utf8(bytes.to_vec())?)),
            PacketFieldType::Bool => Ok(PacketFieldValue::Bool(bytes[0] !=0)),
        }
    }

    ///parses the given string into the field
    /// 
    /// # Errors
    /// 
    /// errors if the input cant be parsed
    pub fn make_from_string(&self, input: &str) -> anyhow::Result<PacketFieldValue>{
        Ok(
            match self{
                PacketFieldType::UnsignedByte |
                PacketFieldType::SignedByte |
                PacketFieldType::UnsignedShort |
                PacketFieldType::SignedShort |
                PacketFieldType::UnsignedTwoFour |
                PacketFieldType::SignedTwoFour |
                PacketFieldType::UnsignedInteger |
                PacketFieldType::SignedInteger |
                PacketFieldType::UnsignedLong |
                PacketFieldType::SignedLong |
                PacketFieldType::Float |
                PacketFieldType::Double => PacketFieldValue::Number(input.parse::<f64>()?),
                PacketFieldType::Bool => PacketFieldValue::Bool(input.to_lowercase() == "true" || input.to_lowercase() == "t"),
                PacketFieldType::String => PacketFieldValue::String(input.to_owned()),
            }
        )
    }
    /// returns the size of the data included within the packetFieldType
    pub fn size(&self) -> anyhow::Result<usize> {
        match self {
            PacketFieldType::UnsignedByte | PacketFieldType::SignedByte => Ok(1),
            PacketFieldType::UnsignedShort | PacketFieldType::SignedShort => Ok(2),
            PacketFieldType::UnsignedInteger | PacketFieldType::SignedInteger => Ok(4),
            PacketFieldType::UnsignedLong | PacketFieldType::SignedLong => Ok(8),
            PacketFieldType::Float => Ok(4),
            PacketFieldType::Double => Ok(8),
            PacketFieldType::UnsignedTwoFour => Ok(3),
            PacketFieldType::SignedTwoFour => Ok(3),
            PacketFieldType::String => Err(anyhow::anyhow!("size of string is unknown from this context")),
            PacketFieldType::Bool => Ok(1),
        }
    }
}

fn slice_to_fixed_size<const N: usize>(slice: &[u8]) -> [u8; N] {
    slice.try_into().expect("Given slice has incorrect length!")
}

#[cfg(test)]
mod tests {
    use super::*;
    //tests that ints can be made
    #[test]
    fn make_0_uint() {
        let unsigned_integer_type = PacketFieldType::SignedInteger;
        let parsed = unsigned_integer_type.make_from_string("0").unwrap();
        assert_eq!(parsed,PacketFieldValue::Number(0.0));
    }

    //tests that signed ints can be made
    #[test]
    fn make_negative_int() {
        let unsigned_integer_type = PacketFieldType::SignedInteger;
        let parsed = unsigned_integer_type.make_from_string("-1").unwrap();
        assert_eq!(parsed,PacketFieldValue::Number(-1.0));
    }

    //test that floats can be made
    #[test]
    fn make_00_float() {
        let unsigned_integer_type = PacketFieldType::Float;
        let parsed = unsigned_integer_type.make_from_string("0.0").unwrap();
        assert_eq!(parsed,PacketFieldValue::Number(0.0));
    }

    //test that floats can be made from non decimal input
    #[test]
    fn make_0_float() {
        let unsigned_integer_type = PacketFieldType::Float;
        let parsed = unsigned_integer_type.make_from_string("0").unwrap();
        assert_eq!(parsed,PacketFieldValue::Number(0.0));
    }

    //test that negative floats can be made
    #[test]
    fn make_negative_float() {
        let unsigned_integer_type = PacketFieldType::Float;
        let parsed = unsigned_integer_type.make_from_string("-1").unwrap();
        assert_eq!(parsed,PacketFieldValue::Number(-1.0));
    }
}
