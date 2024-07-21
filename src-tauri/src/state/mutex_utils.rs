use std::sync::Mutex;

use anyhow::{Error,anyhow};

/// Calls the given callback that uses the state inside the given mutex as its only parameter.
///
/// # Panics
///
/// Panics if the lock is already held by the current thread.
///
/// # Errors
///
/// This function will return an error if the mutex cannot be acquired or the given callback returns an error.
pub fn use_state_in_mutex<State, ReturnType, ErrorType: std::fmt::Display>(
    mutex: &Mutex<State>,
    callback: &mut dyn FnMut(&mut State) -> Result<ReturnType, ErrorType>,
) -> Result<ReturnType, Error>
{
    let locked_mutex_result = mutex.lock();

    if locked_mutex_result.is_err() {
        return Err(anyhow!(locked_mutex_result.err().unwrap().to_string()));
    }

    let state = &mut *locked_mutex_result.unwrap();

    let result = callback(state);

    match result {
        Ok(return_value) => Ok(return_value),
        Err(error) => Err(anyhow!(error.to_string())),//convert error to string and back to error because all errors might need to be turned back into a string later
    }
}
