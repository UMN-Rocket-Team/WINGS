use std::sync::Mutex;

use crate::{state::mutex_utils::use_state_in_mutex, data_processing::DataProcessor};

#[derive(Default)]
pub struct DataProcessorState{
    data_processor: Mutex<DataProcessor>
}

pub fn use_data_processor<ReturnType,ErrorType>(
    data_processor_state: &tauri::State<'_, DataProcessorState>,
    callback: &mut dyn FnMut(&mut DataProcessor) -> Result<ReturnType, ErrorType>,
) -> Result<ReturnType, String> 
    where
    ErrorType: std::fmt::Display,
{
    use_state_in_mutex(&data_processor_state.data_processor, callback)
}
