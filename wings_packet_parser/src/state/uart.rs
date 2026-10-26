use serde::{Deserialize, Serialize};

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
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct SendingState {
    packets_sent: u32,
}

impl SendingState {
    fn starting() -> Self {
        Self { packets_sent: 0 }
    }

    fn sent(packets_sent: u32) -> Self {
        Self { packets_sent }
    }
}