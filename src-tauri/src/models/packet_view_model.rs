//! # Packet View Model
//!
//! This module provides a "view model" representation of a `PacketStructure`.
//!
//! Key components:
//! - `PacketStructureViewModel`: The top-level container for the packet layout.
//! - `PacketComponent`: An enum representing the different parts of the layout
//! - `create_packet_view_model`: Performs the conversion from `PacketStructure` to `PacketStructureViewModel`.

use serde::{Deserialize, Serialize};

use crate::models::packet_structure::{PacketDelimiter, PacketField, PacketStructure};

/// Represents a packet structure in a format suitable for display.
///
/// This struct holds the basic information of a packet and a vector of `PacketComponent`s,
/// which together describe the complete, contiguous layout of the packet, including any gaps.
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
    #[allow(dead_code)]
    pub fn to_packet_structure(&self) -> PacketStructure {
        let mut packet_fields: Vec<PacketField> = Vec::new();
        let mut packet_delimiters: Vec<PacketDelimiter> = Vec::new();
        for component in &self.components {
            match component {
                PacketComponent::Field(field) => packet_fields.push(field.clone()),
                PacketComponent::Delimiter(delimiter) => packet_delimiters.push(PacketDelimiter {
                    index: delimiter.index,
                    name: delimiter.name.to_string(),
                    identifier: hex::decode(&delimiter.identifier).unwrap(), // used unwrap instead of match(program will panic if this cant decode)
                    offset_in_packet: delimiter.offset_in_packet,
                }),
                PacketComponent::Gap(_gap) => {} //gaps are view only and can be ignored
            };
        }
        PacketStructure::make_from_fields_and_delims(
            self.id,
            self.name.clone(),
            packet_fields,
            packet_delimiters,
        )
    }
}

/// An enum to categorize the different types of packet components.
///
/// This is primarily used for deserialization logic where the type of
/// component needs to be identified.
#[derive(Deserialize, Clone, Copy)]
pub enum PacketComponentType {
    Field,
    Delimiter,
    Gap,
}

/// An enum representing a single component in the packet's visual layout.
///
/// This is a tagged enum that can represent a data field, a fixed delimiter,
/// or a gap between other components. The `serde` attributes ensure it's
/// serialized to JSON with a `type` field (e.g., `"type": "Field"`) and a `data` field
/// containing the corresponding struct.
#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum PacketComponent {
    Field(PacketField),
    Delimiter(PacketDelimiterViewModel),
    Gap(PacketGap),
}

impl PacketComponent {
    /// Returns the starting offset of the component within the packet.
    fn get_offset_in_packet(&self) -> usize {
        match self {
            PacketComponent::Field(field) => field.offset_in_packet,
            PacketComponent::Delimiter(delimiter) => delimiter.offset_in_packet,
            PacketComponent::Gap(gap) => gap.offset_in_packet,
        }
    }

    /// Returns the length of the component in bytes.
    fn len(&self) -> usize {
        match self {
            PacketComponent::Field(field) => field.r#type.size().unwrap_or(0),
            // Delimiter identifier is hex encoded, so length is half the string length.
            PacketComponent::Delimiter(delimiter) => delimiter.identifier.len() / 2,
            PacketComponent::Gap(gap) => gap.size,
        }
    }
}

/// A view model representation of a `PacketDelimiter`.
///
/// This struct is similar to the core `PacketDelimiter` but stores the
/// `identifier` as a hex-encoded `String` for easier handling in front-end applications.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PacketDelimiterViewModel {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) identifier: String,
    pub(crate) offset_in_packet: usize,
}

/// Represents a gap in the packet structure.
///
/// A gap is an unallocated space between two other components or before the first component.
/// This is a purely visual concept used to create a complete representation of the packet layout.
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

    // Convert all fields from the data model into `Field` components.
    for field in &packet_structure.fields {
        components.push(PacketComponent::Field(field.clone()));
    }

    // Convert all delimiters from the data model into `Delimiter` components.
    for delimiter in &packet_structure.delimiters {
        components.push(PacketComponent::Delimiter(PacketDelimiterViewModel {
            index: delimiter.index,
            name: delimiter.name.clone(),
            identifier: hex::encode(&delimiter.identifier),
            offset_in_packet: delimiter.offset_in_packet,
        }));
    }

    // Check if there is a gap at the very beginning of the packet.
    if let Some(first_component) = components.first() {
        if first_component.get_offset_in_packet() != 0 {
            components.insert(
                0,
                PacketComponent::Gap(PacketGap {
                    size: first_component.get_offset_in_packet(),
                    offset_in_packet: 0,
                }),
            );
        }
    }

    // Iterate backwards through the components to check for gaps between them.
    for i in (0..components.len().saturating_sub(1)).rev() {
        let component = &components[i];

        let current_component_end = component.get_offset_in_packet() + component.len();
        let next_offset = components[i + 1].get_offset_in_packet();

        // If the end of the current component is before the start of the next one, there's a gap.
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

    PacketStructureViewModel {
        id: packet_structure.id,
        name: packet_structure.name.clone(),
        components,
    }
}
