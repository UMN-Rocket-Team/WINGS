use std::sync::Mutex;

use crate::state::mutex_utils::use_state_in_mutex;

pub fn use_struct<T: Send,ReturnType, ErrorType>(
    inside_struct: &tauri::State<'_, Mutex<T>>,
    callback: &mut dyn FnMut(&mut T) -> Result<ReturnType, ErrorType>,
) -> Result<ReturnType, String>
where
    ErrorType: std::fmt::Display,
{
    use_state_in_mutex(&inside_struct, callback)
}
