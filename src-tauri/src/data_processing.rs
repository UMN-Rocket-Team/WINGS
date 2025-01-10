use std::sync::Mutex;

use crate::models::packet::PacketFieldValue;

#[derive(serde::Serialize, Default, Debug, Clone)]
pub struct DisplayPacket {
    structure_id: usize,
    field_data: Vec<PacketFieldValue>,
}
#[derive(serde::Serialize, Default)]
pub struct DisplayPacketFieldNames {
    structure_id: usize,
    field_names: Vec<String>,
}

/// A `Mutex` wrapper for `DataProcessor`
pub type DataProcessorState = Mutex<DataProcessor>;
#[derive(Default)]
pub struct DataProcessor {
    
}
