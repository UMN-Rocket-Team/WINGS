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
) -> ReturnType {
    //it is ok to panic if a lock fails: https://users.rust-lang.org/t/any-examples-of-recovering-from-a-poisoned-lock/29435
    let mut locked_mutex_result = mutex.lock().expect("Poison!");

    let state = &mut *locked_mutex_result;

    //println!("unlocking! {}", std::any::type_name::<State>());
    callback(state)
}
