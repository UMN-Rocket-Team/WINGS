use serde::{Deserialize, Serialize};

use crate::models::packet::Packet;

/// Sending Modes

#[derive(Deserialize, Serialize, Clone, Debug, Copy)]
pub enum SendingModes {
    FromCSV,
    AllZeros,
    AllOnes,
    Alternating,
    TimeStampAndIncreasing, // assumes that the first field is timestamp compatible
}

/// The object sent to the frontend.
#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct SendingState {
    packet_buffer: Vec<Packet>,
    packets_sent: u32,
    is_dirty: bool,

}

impl SendingState {
    fn sent(&self) -> u32 {
        self.packets_sent
    }

    fn is_dirty(&self) -> bool {
        self.is_dirty
    }
}
