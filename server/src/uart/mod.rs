use std::{cell::OnceCell, sync::Mutex};

use wings_packet_parser::prelude::*;

pub mod receiving_loop;
pub mod sending_loop;

pub const UART_STATE: OnceCell<Mutex<UArtState>> = OnceCell::<Mutex::<UArtState>>::new();

pub struct UArtState<'a> {
    communication_manager: &'a CommunicationManagerState,
    data_processor: &'a DataProcessorState,
    file_handling: &'a FileHandlingState
}

impl <'a> UArtState<'a> {
    pub fn new(
        communication_manager: &'a CommunicationManagerState,
        data_processor: &'a DataProcessorState,
        file_handling: &'a FileHandlingState

    ) -> Self {
        Self {
            communication_manager,
            data_processor,
            file_handling
        }
    }

    pub fn communication_manager(&'a self) -> &'a CommunicationManagerState {
        self.communication_manager
    }

    pub fn data_processor(&'a self) -> &'a DataProcessorState {
        self.data_processor
    }

    pub fn file_handling(&'a self) ->  &'a FileHandlingState {
        self.file_handling
    }
}