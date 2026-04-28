use crate::communication_drivers::aim_parser::AimParser;
use crate::communication_drivers::featherweight_parser::FeatherweightParser;
use crate::communication_drivers::midwest_parser::MidwestParser;
use crate::communication_drivers::serial_packet_parser::SerialPacketParser;
use crate::communication_drivers::teledongle_packet_parser::AltosPacketParser;
use crate::models::packet::Packet;
use crate::models::product::ProductName;
use crate::packet_structure_manager::PacketStructureManager;

pub trait PacketParser: Send {
    fn new() -> Self
    where
        Self: Sized;

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

    /// Register packet structures needed by this parser.
    fn register_packet_structures(
        _packet_structure_manager: &mut PacketStructureManager,
    ) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }
}

/**
 * Factory function to create a packet parser based on the product name.
 * Returns a boxed PacketParser for the selected supported product.
 */
pub fn create_parser_from_product_name(
    parser_name: ProductName,
) -> Box<dyn PacketParser + 'static> {
    match parser_name {
        ProductName::AltusMetrum => Box::new(AltosPacketParser::new()),
        ProductName::Rfd => Box::new(SerialPacketParser::new()),
        ProductName::Featherweight => Box::new(FeatherweightParser::new()),
        ProductName::Aim => Box::new(AimParser::new()),
        ProductName::Midwest => Box::new(MidwestParser::new()),
    }
}

/*
 * Function to register packet structures for a given product's parser.
 * This should be called during initialization to ensure the packet structure manager
 * has the necessary structures for parsing incoming data.
*/
pub fn register_packet_structures_for_product(
    parser_name: ProductName,
    packet_structure_manager: &mut PacketStructureManager,
) -> anyhow::Result<()> {
    match parser_name {
        ProductName::AltusMetrum => {
            AltosPacketParser::register_packet_structures(packet_structure_manager)
        }
        ProductName::Rfd => {
            SerialPacketParser::register_packet_structures(packet_structure_manager)
        }
        ProductName::Featherweight => {
            FeatherweightParser::register_packet_structures(packet_structure_manager)
        }
        ProductName::Aim => AimParser::register_packet_structures(packet_structure_manager),
        ProductName::Midwest => MidwestParser::register_packet_structures(packet_structure_manager),
    }
}
