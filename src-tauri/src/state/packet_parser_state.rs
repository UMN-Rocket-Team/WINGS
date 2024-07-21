use std::sync::Mutex;

use crate::{state::mutex_utils::use_state_in_mutex, packet_parser::PacketParser};

#[derive(Default)]
pub struct PacketParserState {
    pub(crate) packet_parser: Mutex<PacketParser>,
}

pub fn use_packet_parser<ReturnType, ErrorType>(
    packet_parser_state: &tauri::State<'_, PacketParserState>,
    callback: &mut dyn FnMut(&mut PacketParser) -> Result<ReturnType, ErrorType>,
) -> Result<ReturnType, String>
where
    ErrorType: std::fmt::Display,
{
    use_state_in_mutex(&packet_parser_state.packet_parser, callback)
}
