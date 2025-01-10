
use std::sync::Mutex;

use crate::state::mutex_utils::use_state_in_mutex;


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
