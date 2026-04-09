use crate::models::packet::Packet;
use crate::packet_structure_manager::PacketStructureManager;

pub trait PacketParser {
    fn push_data(&mut self, data: &[u8], print_flag: bool);

    fn parse_packets(
        &mut self,
        packet_structure_manager: &PacketStructureManager,
        print_flag: bool,
    ) -> anyhow::Result<Vec<Packet>>;
}
