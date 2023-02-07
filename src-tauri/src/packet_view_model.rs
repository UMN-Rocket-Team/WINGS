use serde::{Deserialize, Serialize};

use crate::packet_structure::{PacketField, PacketStructure};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PacketViewModel {
    id: usize,
    name: String,
    components: Vec<PacketComponent>,
}

#[derive(Serialize, Clone)]
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
            PacketComponent::Delimiter(delimiter) => delimiter.identifier.len(),
            PacketComponent::Gap(gap) => gap.size,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PacketDelimiterViewModel {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) identifier: String,
    pub(crate) offset_in_packet: usize,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PacketGap {
    index: usize,
    size: usize,
    offset_in_packet: usize,
}

pub fn create_packet_view_model(packet_structure: &PacketStructure) -> PacketViewModel {
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

    let mut gap_index: usize = 0;

    for i in 0..(components.len() - 1) {
        let component = &components[i];

        let current_component_end = component.get_offset_in_packet() + component.len();
        let next_offset = components[i + 1].get_offset_in_packet();

        if current_component_end < next_offset {
            components.insert(
                i + 1,
                PacketComponent::Gap(PacketGap {
                    index: gap_index,
                    size: next_offset - current_component_end,
                    offset_in_packet: current_component_end,
                }),
            );
            gap_index += 1;
        }
    }

    return PacketViewModel {
        id: packet_structure.id,
        name: packet_structure.name.clone(),
        components,
    };
}
