use std::sync::Mutex;

use crate::{test::TestManager, mutex_utils::use_state_in_mutex};

#[derive(Default)]
pub struct TestManagerState {
    test_manager: Mutex<TestManager>
}

pub fn use_test_manager<ReturnType>(
    test_manager_state: tauri::State<'_, TestManagerState>,
    callback: &mut dyn FnMut(&mut TestManager) -> Result<ReturnType, anyhow::Error>,
) -> Result<ReturnType, String> {
    use_state_in_mutex(&test_manager_state.test_manager, callback)
}
