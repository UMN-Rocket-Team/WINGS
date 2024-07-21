use std::sync::Mutex;

use anyhow::Error;

use crate::{communication_manager::CommunicationManager, data_processing::DataProcessor, file_handling::FileHandler, packet_parser::PacketParser, packet_structure_manager::PacketStructureManager, sending_loop::SendingLoop, state::mutex_utils::use_state_in_mutex};

pub type CommunicationManagerState = Mutex<CommunicationManager>;
pub type PacketParserState = Mutex<PacketParser>;
pub type FileHandlingState = Mutex<FileHandler>;
pub type SendingLoopState = Mutex<SendingLoop>;
pub type PacketStructureManagerState = Mutex<PacketStructureManager>;
pub type DataProcessorState = Mutex<DataProcessor>;


pub fn use_struct<Struct: Send,ReturnType ,ErrorType: std::fmt::Display>(
    inside_struct: &tauri::State<'_, Mutex<Struct>>,
    callback: &mut dyn FnMut(&mut Struct) -> Result<ReturnType, ErrorType>,
) -> Result<ReturnType, Error>
{
    use_state_in_mutex(&inside_struct, callback)
}

pub fn result_to_string<ReturnType,ErrorType: std::fmt::Display>(
    use_struct_result: Result<ReturnType,ErrorType>
)->Result<ReturnType,String>{
    match  use_struct_result{
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.to_string()),
    }
}

