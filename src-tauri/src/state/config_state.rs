use std::sync::Mutex;

use crate::{config_struct::ConfigStruct, state::mutex_utils::use_state_in_mutex};

#[derive(Default)]
pub struct ConfigState{
    config: Mutex<ConfigStruct>
}

pub fn use_config<ReturnType,ErrorType>(
    config_state: &tauri::State<'_, ConfigState>,
    callback: &mut dyn FnMut(&mut ConfigStruct) -> Result<ReturnType, ErrorType>,
) -> Result<ReturnType, String> 
    where
    ErrorType: std::fmt::Display,
{
    use_state_in_mutex(&config_state.config, callback)
}
