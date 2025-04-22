use std::sync::Mutex;

/// Calls the given callback that uses the state inside the given mutex as its only parameter.
///
/// # Panics
///
/// Panics if the lock is already held by the current thread.
///
/// # Errors
///
/// This function will return an error if the mutex cannot be acquired or the given callback returns an error.
pub fn use_state_in_mutex<State, ReturnType>(
    mutex: &Mutex<State>,
    callback: &mut dyn FnMut(&mut State) -> ReturnType,
) -> Result<ReturnType, String>
{
    //println!("locking! {:?}", std::any::type_name::<State>());
    let locked_mutex_result = mutex.lock();

    if locked_mutex_result.is_err() {
        return Err(locked_mutex_result.err().unwrap().to_string());
    }

    let state = &mut *locked_mutex_result.unwrap();
    let result = callback(state);
    //println!("unlocking! {}", std::any::type_name::<State>());
    Ok(result)
}
