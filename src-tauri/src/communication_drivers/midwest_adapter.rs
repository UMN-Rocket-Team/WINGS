use std::{
    str::from_utf8,
    sync::{Arc, Mutex},
};

use anyhow::bail;

use crate::{
    communication_manager::CommsIF,
    models::{packet::Packet, packet_structure::PacketStructure},
    packet_structure_manager::PacketStructureManager,
    state::mutex_utils::use_state_in_mutex,
};

use super::midwest_parser::AltosPacketParser;
const PRINT_PARSING: bool = false;

#[derive(Default)]
pub struct MidwestAdapter {
    port: Option<Box<dyn serialport::SerialPort>>,
    packet_parser: AltosPacketParser,
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
            println!("Creating Midwest!");
            //################################
            //Midwest Hardcoded packets start here
            //################################

            // Midwest BNO Data Packet.
let mut midwest_bno_structure = PacketStructure::default();
    midwest_bno_structure.ez_make(
        "ba5eba11 F32 F32 F32 F32 F32 F32 F32 F32 F32 ca11ab1e",
        &[
            "Timestamp",
            "state",
            "pkt_len",
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
        .expect("Failed to register test packet");

        // Midwest Alt Data Packet.
    let mut midwest_alt_structure = PacketStructure::default();
    midwest_alt_structure.ez_make(
        "ba5eba11 F32 F32 ca11ab1e",
        &["temperature", "pressure"],
        true,
    );
    midwest_alt_structure.name = "midwest_alt".to_owned();
    ps_manager
        .register_packet_structure(&mut midwest_alt_structure)
        .expect("Failed to register test packet");


            // Midwest GPS Data Packet.
let mut midwest_gps_structure = PacketStructure::default();
    midwest_gps_structure.ez_make(
        "ba5eba11 u32 u8 u8 u8 _1 i32 u32 F32 F32 u32 u32 u8 _2 u32 u32 F32 i32 i32 i32 u32 ca11ab1e",
    &[
        "time_of_week",
        "time_hour",
        "time_min",
        "time_sec",
        "time_nanosec",
        "timeAccuracy",
        "pos_lat",
        "pos_lon",
        "height_msl",
        "height_elip",
        "fixType",
        "numSatellites",
        "verticalAccuracy",
        "horizontalAccuracy",
        "pDOP",
        "vel_north",
        "vel_east",
        "vel_down",
        "vel_accuracy"],true);
    midwest_gps_structure.name = "midwest_gps".to_owned();
    ps_manager
        .register_packet_structure(&mut midwest_gps_structure)
        .expect("Failed to register test packet");

        // Midwest Control Telemetry Data Packet.
    let mut midwest_control_telemetry_structure = PacketStructure::default();
    midwest_control_telemetry_structure.ez_make(
        "ba5eba11 F32 F32 ca11ab1e",
        &["PD_error", "loop_update_rule"],
        true,
    );
    midwest_control_telemetry_structure.name = "midwest_control_telemetry".to_owned();
    ps_manager
        .register_packet_structure(&mut midwest_control_telemetry_structure)
        .expect("Failed to register test packet");
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
    fn init_device(&mut self, port_name: &str, _baud: u32) -> anyhow::Result<()> {
        if port_name.is_empty() {
            self.port = None;
        } else {
            self.baud = 9600;
            let mut new_port = serialport::new(port_name, self.baud)
                .flow_control(serialport::FlowControl::None)
                .open()?;
            // Short non-zero timeout is needed to receive data from the serialport when
            // the buffer isn't full yet.
            new_port.set_timeout(std::time::Duration::from_millis(1))?;
            self.port = Some(new_port);

            //setup commands for the radio
            self.write_port(&[0x7E, 0x0A, 0x45, 0x20, 0x30, 0x0A, 0x6D, 0x20, 0x30, 0x0A])?;
            self.parse_device_data(&mut vec![], &mut vec![])?;
            self.write_port(&[
                0x6D, 0x20, 0x32, 0x30, 0x0A, 0x6D, 0x20, 0x30, 0x0A, 0x63, 0x20, 0x73, 0x0A, 0x66,
                0x0A, 0x76, 0x0A,
            ])?;
            self.parse_device_data(&mut vec![], &mut vec![])?;
            self.write_port(&[
                0x6D, 0x20, 0x32, 0x30, 0x0A, 0x6D, 0x20, 0x30, 0x0A, 0x63, 0x20, 0x46, 0x20, 0x34,
                0x33, 0x35, 0x30, 0x35, 0x30, 0x0A, 0x6D, 0x20, 0x32, 0x30, 0x0A, 0x6D, 0x20, 0x30,
                0x0A, 0x6D, 0x20, 0x32, 0x30, 0x0A, 0x6D, 0x20, 0x30, 0x0A, 0x63, 0x20, 0x54, 0x20,
                0x30, 0x0A, 0x6D, 0x20, 0x32, 0x30, 0x0A, 0x6D, 0x20, 0x32, 0x30, 0x0A,
            ])?;
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
            None => bail!("No active test port"),
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
        let _bytes_read = active_port.read(&mut buffer)?;
        let str = from_utf8(&buffer)?;
        let mut parsed_str = "".to_owned();
        for c in str.chars() {
            if c.is_ascii_hexdigit() {
                parsed_str.push(c);
            }
        }
        data_vector.append(&mut hex::decode(parsed_str)?);
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
