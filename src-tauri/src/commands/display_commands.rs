use std::time::Duration;

use crate::state::sending_loop_state::{SendingLoopState, use_sending_loop_manager};

#[derive(Default)]

pub struct DisplayCommands {
    data: Vec<u8>,
}