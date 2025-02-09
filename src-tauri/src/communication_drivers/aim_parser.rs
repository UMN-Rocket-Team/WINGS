use std::cmp::max;

use crate::{
    models::{self, packet::{Packet, PacketFieldValue}},
    packet_structure_manager::PacketStructureManager,
};

#[derive(Default)]

pub struct AimParser {
}

/// responsible converting raw data to packets
impl AimParser {

    /// processes the raw data queue, returning a Vector(aka. array) of the processed packets
    pub fn parse_transmission(
        &mut self,
        packet_structure_manager: &PacketStructureManager,
        transmission: [u8;65],
        print_flag: bool,
    ) -> Vec<Packet> {
        todo!()
    }
}

