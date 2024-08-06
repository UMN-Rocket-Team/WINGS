use std::sync::Mutex;

use anyhow::{anyhow,Error};

use crate::{communication_manager::CommunicationManager, config_struct::ConfigStruct, data_processing::DataProcessor, file_handling::FileHandler, packet_parser::PacketParser, packet_structure_manager::PacketStructureManager, sending_loop::SendingLoop, state::mutex_utils::use_state_in_mutex};

pub type CommunicationManagerState = Mutex<CommunicationManager>;
pub type PacketParserState = Mutex<PacketParser>;
pub type FileHandlingState = Mutex<FileHandler>;
pub type SendingLoopState = Mutex<SendingLoop>;
pub type PacketStructureManagerState = Mutex<PacketStructureManager>;
pub type DataProcessorState = Mutex<DataProcessor>;
pub type ConfigState = ConfigStruct;


pub fn use_struct<Struct: Send,ReturnType>(
    inside_struct: &tauri::State<'_, Mutex<Struct>>,
    callback: &mut dyn FnMut(&mut Struct) -> ReturnType,
) -> Result<ReturnType,String>
{
    use_state_in_mutex(&inside_struct, callback)
}

pub fn result_to_string<ReturnType,ErrorType: std::fmt::Display>(
    use_struct_result: Result<Result<ReturnType,ErrorType>,String>
)->Result<ReturnType,String>{
    match  use_struct_result{
        Ok(ok) => {
            match  ok{
                Ok(ok_2) => Ok(ok_2),
                Err(err_2) => Err(err_2.to_string()),
            }
        },
        Err(err) => Err(err),
    }
}
pub fn result_to_error<ReturnType,ErrorType: std::fmt::Display>(
    use_struct_result: Result<Result<ReturnType,ErrorType>,String>
)->Result<ReturnType,Error>{
    match  use_struct_result{
        Ok(ok) => {
            match  ok{
                Ok(ok_2) => Ok(ok_2),
                Err(err_2) => Err(anyhow!(err_2.to_string())),
            }
        },
        Err(err) => Err(anyhow!(err)),
    }
}

