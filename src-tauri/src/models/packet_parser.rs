use crate::models::packet::Packet;
use crate::packet_structure_manager::PacketStructureManager;

pub trait PacketParser: Send {
    /// Get a mutable reference to the unparsed data vector, which is where incoming data is stored
    /// before being parsed into packets.
    fn get_unparsed_data(&mut self) -> &mut Vec<u8>;

    /// push data to unparsed data vector, and print it if print_flag is true
    fn push_data(&mut self, data: &[u8], print_flag: bool) {
        let unparsed_data = self.get_unparsed_data();
        unparsed_data.extend(data);
        if print_flag {
            println!("Unparsed data: {:02X?}", unparsed_data);
        }
    }

    /// Parse packets from unparsed data, using the packet structure manager to determine how to parse the data, 
    /// and return a vector of parsed packets. If print_flag is true, print the parsed packets.
    fn parse_packets(
        &mut self,
        packet_structure_manager: &PacketStructureManager,
        print_flag: bool,
    ) -> anyhow::Result<Vec<Packet>>;
}

pub struct PacketParserStruct {
    unparsed_data: Vec<u8>,
}
