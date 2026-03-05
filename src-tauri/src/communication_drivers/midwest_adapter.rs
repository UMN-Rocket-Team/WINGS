use std::sync::{Arc, Mutex};

use anyhow::bail;

use crate::{
    communication_manager::CommsIF,
    models::{packet::Packet, packet_structure::PacketStructure},
    packet_structure_manager::PacketStructureManager,
    state::mutex_utils::use_state_in_mutex,
};

use super::serial_packet_parser::SerialPacketParser;
const PRINT_PARSING: bool = false;

pub fn register_midwest_packet_structures(
    ps_manager: &mut PacketStructureManager,
) -> anyhow::Result<()> {
    if ps_manager
        .packet_structures
        .iter()
        .any(|packet_structure| packet_structure.name == "midwest_bno")
    {
        return Ok(());
    }

    if PRINT_PARSING {
        println!("Creating Midwest!");
    }

    // Midwest BNO Data Packet.
    let mut midwest_bno_structure = PacketStructure::default();
    midwest_bno_structure.ez_make(
        "ba5eba11 u32 02 u8 0034 F32 F32 F32 F32 F32 F32 F32 F32 F32 ca11ab1e",
        &[
            "timestamp",
            "rocket_state",
            "acc_x",
            "acc_y",
            "acc_z",
            "gyro_x",
            "gyro_y",
            "gyro_z",
            "eul_heading",
            "eul_roll",
            "eul_pitch",
        ],
        true,
    );
    midwest_bno_structure.name = "midwest_bno".to_owned();
    ps_manager
        .register_packet_structure(&mut midwest_bno_structure)
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    // Midwest Alt Data Packet.
    let mut midwest_alt_structure = PacketStructure::default();
    midwest_alt_structure.ez_make(
        "ba5eba11 u32 04 u8 0018 F32 F32 ca11ab1e",
        &["timestamp", "rocket_state", "temperature", "pressure"],
        true,
    );
    midwest_alt_structure.name = "midwest_alt".to_owned();
    ps_manager
        .register_packet_structure(&mut midwest_alt_structure)
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    // Midwest GPS Data Packet.
    let mut midwest_gps_structure = PacketStructure::default();
    midwest_gps_structure.ez_make(
        "ba5eba11 u32 08 u8 0028 u32 F32 F32 u32 u8 u8 F32 ca11ab1e",
        &[
            "timestamp",
            "rocket_state",
            "time_of_week",
            "pos_lat",
            "pos_lon",
            "height_msl",
            "fixType",
            "numSatellites",
            "pDOP",
        ],
        true,
    );
    midwest_gps_structure.name = "midwest_gps".to_owned();
    ps_manager
        .register_packet_structure(&mut midwest_gps_structure)
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    // Midwest Control Telemetry Data Packet.
    let mut midwest_control_telemetry_structure = PacketStructure::default();
    midwest_control_telemetry_structure.ez_make(
        "ba5eba11 u32 11 u8 0028 F32 F32 F32 F32 F32 F32 ca11ab1e",
        &[
            "timestamp",
            "rocket_state",
            "PD_error",
            "loop_update_rule",
            "target_pos",
            "model_vel",
            "model_theta",
            "model_servo_command",
        ],
        true,
    );
    midwest_control_telemetry_structure.name = "midwest_control_telemetry".to_owned();
    ps_manager
        .register_packet_structure(&mut midwest_control_telemetry_structure)
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    Ok(())
}

#[derive(Default)]
pub struct MidwestAdapter {
    port: Option<Box<dyn serialport::SerialPort>>,
    packet_parser: SerialPacketParser,
    baud: u32,
    id: usize,
    packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
}

impl CommsIF for MidwestAdapter {
    ///creates a new instance of a comms device with the given packet structure manager
    fn new(packet_structure_manager: Arc<Mutex<PacketStructureManager>>) -> Self
    where
        Self: Sized,
    {
        use_state_in_mutex(&packet_structure_manager, &mut |ps_manager| {
            if let Err(err) = register_midwest_packet_structures(ps_manager) {
                eprintln!("Failed to register Midwest packet structures: {err}");
            }
        });
        MidwestAdapter {
            port: None,
            packet_parser: Default::default(),
            baud: 0,
            id: 0,
            packet_structure_manager,
        }
    }

    /// Set the path of the active port
    /// If path is empty, any active port is closed
    ///
    /// # Errors
    ///
    /// Returns an error if port_name is invalid, or if unable to clear the device buffer
    fn init_device(&mut self, port_name: &str, baud: u32) -> anyhow::Result<()> {
        if port_name.is_empty() {
            self.port = None;
        } else {
            self.baud = baud;
            let mut new_port = serialport::new(port_name, self.baud).open()?;
            new_port.clear(serialport::ClearBuffer::All)?;
            // Short non-zero timeout is needed to receive data from the serialport when
            // the buffer isn't full yet.
            new_port.set_timeout(std::time::Duration::from_millis(1))?;
            self.port = Some(new_port);
        }
        Ok(())
    }

    /// Returns true if there is an active port
    fn is_init(&self) -> bool {
        self.port.is_some()
    }

    /// Attempt to write bytes to the radio test port
    ///
    /// # Errors
    ///
    /// returns an error if there is no active port
    fn write_port(&mut self, packet: &[u8]) -> anyhow::Result<()> {
        let port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No serial port initialized for Midwest adapter."),
        };

        port.write_all(packet)?;

        Ok(())
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_type(&self) -> String {
        "Midwest".to_owned()
    }

    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        let active_port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No read port has been set"),
        };

        let mut buffer = [0; 4096];
        let bytes_read = active_port.read(&mut buffer)?;
        data_vector.extend_from_slice(&buffer[..bytes_read]);
        Ok(())
    }

    fn parse_device_data(
        &mut self,
        data_vector: &mut Vec<u8>,
        packet_vector: &mut Vec<Packet>,
    ) -> anyhow::Result<()> {
        self.packet_parser.push_data(data_vector, PRINT_PARSING);
        use_state_in_mutex(
            &self.packet_structure_manager,
            &mut |ps_manager| -> anyhow::Result<()> {
                packet_vector.extend_from_slice(
                    &self
                        .packet_parser
                        .parse_packets(ps_manager, PRINT_PARSING)?,
                );
                Ok(())
            },
        )?;
        Ok(())
    }
}
