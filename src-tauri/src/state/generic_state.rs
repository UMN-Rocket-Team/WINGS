
use std::sync::Mutex;

use anyhow::{anyhow,Error};

use crate::{communication_manager::CommunicationManager, file_handling::{log_handlers::LogHandler,config_struct::ConfigStruct}, data_processing::DataProcessor, sending_loop::SendingLoop, state::mutex_utils::use_state_in_mutex};

pub type CommunicationManagerState = Mutex<CommunicationManager>;
pub type FileHandlingState = Mutex<LogHandler>;
pub type SendingLoopState = Mutex<SendingLoop>;
pub type DataProcessorState = Mutex<DataProcessor>;
pub type ConfigState = Mutex<ConfigStruct>;


pub fn use_struct<Struct: Send,ReturnType>(
    state_to_use: &tauri::State<'_, Mutex<Struct>>,
    callback: &mut dyn FnMut(&mut Struct) -> ReturnType,
) -> Result<ReturnType,String>
{
    use_state_in_mutex(&state_to_use, callback)
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

//gets a clone of the object within the given mutex, or gives an error
pub fn get_clone<Struct: Clone + Send>(
    state_to_clone: &tauri::State<'_, Mutex<Struct>>,
) -> Result<Struct,Error>
{
   match use_state_in_mutex(&state_to_clone, &mut|unwrapped_state|{unwrapped_state.clone()}){
        Ok(ok) => Ok(ok),
        Err(err) => Err(anyhow!(err)),
   }
}
