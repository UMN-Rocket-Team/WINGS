use serde::{Deserialize, Serialize};

use crate::models::packet_structure::{PacketDelimiter, PacketField, PacketStructure};

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PacketStructureViewModel {
    id: usize,
    name: String,
    components: Vec<PacketComponent>,
}

impl PacketStructureViewModel {

    /// Takes current PacketStructureViewModel and parses it into a packetStructure which is then returned
    ///
    /// ### Output
    /// * 'PacketStructure' - contains all data from the PacketStructureViewModel, repackaged as a PacketStructure
    pub fn to_packet_structure(&self) -> PacketStructure {
        let mut packet_fields: Vec<PacketField> = Vec::new();
        let mut packet_delimiters: Vec<PacketDelimiter> = Vec::new();
        for component in &self.components {
            match component {
                PacketComponent::Field(field) => packet_fields.push(field.clone()),
                PacketComponent::Delimiter(delimiter) => packet_delimiters.push(PacketDelimiter {
                    index: delimiter.index,
                    name: delimiter.name.to_string(),
                    identifier: hex::decode(delimiter.identifier.to_string()).unwrap(), // used unwrap instead of match(program will panic if this cant decode)
                    offset_in_packet: delimiter.offset_in_packet,
                }),
                PacketComponent::Gap(_gap) => {} //gaps are view only and can be ignored
            };
        }
        return PacketStructure {
            id: self.id,
            name: self.name.clone(),
            fields: packet_fields,
            delimiters: packet_delimiters,
            metafields: vec![],
            packet_crc: vec![]
        };
    }
}

#[derive(Deserialize, Clone, Copy)]
pub enum PacketComponentType {
    Field,
    Delimiter,
    Gap,
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum PacketComponent {
    Field(PacketField),
    Delimiter(PacketDelimiterViewModel),
    Gap(PacketGap),
}

impl PacketComponent {
    fn get_offset_in_packet(&self) -> usize {
        match self {
            PacketComponent::Field(field) => field.offset_in_packet,
            PacketComponent::Delimiter(delimiter) => delimiter.offset_in_packet,
            PacketComponent::Gap(gap) => gap.offset_in_packet,
        }
    }

    fn len(&self) -> usize {
        match self {
            PacketComponent::Field(field) => field.r#type.size(),
            PacketComponent::Delimiter(delimiter) => delimiter.identifier.len() / 2,
            PacketComponent::Gap(gap) => gap.size,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PacketDelimiterViewModel {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) identifier: String,
    pub(crate) offset_in_packet: usize,
}

#[derive(Serialize, Clone, Deserialize, Copy, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PacketGap {
    size: usize,
    offset_in_packet: usize,
}

/// Takes a Packet Structure and parses it and returns it as a Packet View Model
/// 
/// ### Arguments
/// * 'packet_structure' - contains a packet structure that needs to be put into a displayable format
/// ### Output
/// * 'PacketStructureViewModel' contains the packet structure that was given in a new format
pub fn create_packet_view_model(packet_structure: &PacketStructure) -> PacketStructureViewModel {
    let mut components: Vec<PacketComponent> =
        Vec::with_capacity(packet_structure.delimiters.len() + packet_structure.fields.len());

    for field in &packet_structure.fields {
        components.push(PacketComponent::Field(field.clone()));
    }

    for delimiter in &packet_structure.delimiters {
        components.push(PacketComponent::Delimiter(PacketDelimiterViewModel {
            index: delimiter.index,
            name: delimiter.name.clone(),
            identifier: hex::encode(&delimiter.identifier),
            offset_in_packet: delimiter.offset_in_packet,
        }));
    }

    components.sort_by(|lhs, rhs| lhs.get_offset_in_packet().cmp(&rhs.get_offset_in_packet()));

    if let Some(first_component) = components.get(0) {
        if first_component.get_offset_in_packet() != 0 {
            components.insert(0, PacketComponent::Gap(PacketGap {
                size: first_component.get_offset_in_packet(),
                offset_in_packet: 0
            }));
        }
    }

    // This loop checks for a gap *after* the component at index `i`.
    // The last component by definition can't have a gap after it.
    // Must iterate backwards because we are adding items as we loop.
    for i in (0..(components.len() - 1)).rev() {
        let component = &components[i];

        let current_component_end = component.get_offset_in_packet() + component.len();
        let next_offset = components[i + 1].get_offset_in_packet();

        if current_component_end < next_offset {
            components.insert(
                i + 1,
                PacketComponent::Gap(PacketGap {
                    size: next_offset - current_component_end,
                    offset_in_packet: current_component_end,
                }),
            );
        }
    }

    return PacketStructureViewModel {
        id: packet_structure.id,
        name: packet_structure.name.clone(),
        components,
    };
}

#[cfg(test)]
mod tests {
    use crate::models::{packet_structure::{PacketDelimiter, PacketStructure}, packet_view_model::{PacketComponent, PacketDelimiterViewModel, PacketGap, PacketStructureViewModel}};

    use super::create_packet_view_model;

    #[test]
    fn delimiter() {
        let packet_structure = PacketStructure {
            id: 0,
            name: String::from("Test packet"),
            fields: vec![],
            delimiters: vec![
                PacketDelimiter {
                    index: 0,
                    name: String::from("Test delimiter"),
                    identifier: vec![0xde, 0xad, 0xbe, 0xef],
                    offset_in_packet: 0
                }
            ],
            metafields: vec![],
            packet_crc: vec![]
        };
        let view_model = create_packet_view_model(&packet_structure);
        assert_eq!(view_model, PacketStructureViewModel {
            id: 0,
            name: String::from("Test packet"),
            components: vec![
                PacketComponent::Delimiter(PacketDelimiterViewModel {
                    index: 0,
                    name: String::from("Test delimiter"),
                    identifier: String::from("deadbeef"),
                    offset_in_packet: 0
                })
            ]
        });
        assert_eq!(view_model.components[0].len(), 4); // 4 bytes long
    }

    #[test]
    fn starts_with_gap() {
        let packet_structure = PacketStructure {
            id: 0,
            name: String::from("Test packet"),
            fields: vec![],
            delimiters: vec![
                // There is a gap before the first field/delimiter
                PacketDelimiter {
                    index: 0,
                    name: String::from("Test delimiter 1"),
                    identifier: vec![0x34],
                    offset_in_packet: 10
                },
                // There is also a gap between these, which we will use to test that the
                // gap index is updated correctly
                PacketDelimiter {
                    index: 1,
                    name: String::from("Test delimiter 2"),
                    identifier: vec![0x56],
                    offset_in_packet: 12
                }
            ],
            metafields: vec![],
            packet_crc: vec![]
        };
        let view_model = create_packet_view_model(&packet_structure);
        assert_eq!(view_model, PacketStructureViewModel {
            id: 0,
            name: String::from("Test packet"),
            components: vec![
                PacketComponent::Gap(PacketGap {
                    size: 10,
                    offset_in_packet: 0
                }),
                PacketComponent::Delimiter(PacketDelimiterViewModel {
                    index: 0,
                    name: String::from("Test delimiter 1"),
                    identifier: String::from("34"),
                    offset_in_packet: 10
                }),
                PacketComponent::Gap(PacketGap {
                    size: 1,
                    offset_in_packet: 11
                }),
                PacketComponent::Delimiter(PacketDelimiterViewModel {
                    index: 1,
                    name: String::from("Test delimiter 2"),
                    identifier: String::from("56"),
                    offset_in_packet: 12
                })
            ]
        });
    }

    #[test]
    pub fn a_lot_of_gaps() {
        // There used to be a bug where when there were a ton of gaps, the last few packets
        // would not be checked for gaps.
        let make_structure_delimiter = |i: u8| PacketDelimiter {
            index: i as usize,
            name: i.to_string(),
            identifier: vec![i],
            offset_in_packet: i as usize
        };
        let packet_structure = PacketStructure {
            id: 0,
            name: String::from("Test packet"),
            fields: vec![],
            delimiters: vec![
                make_structure_delimiter(0),
                make_structure_delimiter(2),
                make_structure_delimiter(4),
                make_structure_delimiter(6),
                make_structure_delimiter(8),
                make_structure_delimiter(10),
                make_structure_delimiter(12),
                make_structure_delimiter(14),
            ],
            metafields: vec![],
            packet_crc: vec![]
        };

        let view_model = create_packet_view_model(&packet_structure);
        let make_view_gap = |i: u8| PacketComponent::Gap(PacketGap {
            size: 1,
            offset_in_packet: i as usize
        });
        let make_view_delimiter = |i: u8| PacketComponent::Delimiter(PacketDelimiterViewModel {
            index: i as usize,
            name: i.to_string(),
            identifier: hex::encode(vec![i]),
            offset_in_packet: i as usize
        });
        assert_eq!(view_model, PacketStructureViewModel {
            id: 0,
            name: String::from("Test packet"),
            components: vec![
                make_view_delimiter(0),
                make_view_gap(1),
                make_view_delimiter(2),
                make_view_gap(3),
                make_view_delimiter(4),
                make_view_gap(5),
                make_view_delimiter(6),
                make_view_gap(7),
                make_view_delimiter(8),
                make_view_gap(9),
                make_view_delimiter(10),
                make_view_gap(11),
                make_view_delimiter(12),
                make_view_gap(13),
                make_view_delimiter(14),
            ]
        });
    }
}
