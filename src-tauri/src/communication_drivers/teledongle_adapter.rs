use std::{str::from_utf8, sync::{Arc, Mutex}};

use anyhow::bail;

use crate::{communication_manager::CommsIF, models::{packet::Packet, packet_structure::PacketStructure}, packet_structure_manager::PacketStructureManager, state::mutex_utils::use_state_in_mutex};

use super::teledongle_packet_parser::AltosPacketParser;
const PRINT_PARSING: bool = false;

#[derive(Default)]
pub struct TeleDongleAdapter {
    port: Option<Box<dyn serialport::SerialPort>>,
    packet_parser: AltosPacketParser,
    baud: u32,
    id: usize,
    packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
}

impl CommsIF for TeleDongleAdapter {

     ///creates a new instance of a comms device with the given packet structure manager
     fn new(
        packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
    ) -> Self 
    where
        Self: Sized {
            
        _ = use_state_in_mutex(&packet_structure_manager, &mut |ps_manager| {
           
            //################################
            //Altus Metrum Hardcoded packets start here
            //################################

            // TeleMega IMU Data Packet.
            let mut telemega_imu_data = PacketStructure::default();
            telemega_imu_data.ez_make("_2 u16 08 u8 i16 i32 i16 i16 i16 i16 i16 i16 i16 i16 i16 i16", 
                &["Timestamp",
                "Angle from vertical in degrees","accel","pressure (Pa * 10)","temperature (°C * 100)",
                "accel_x","accel_y","accel_z",
                "gyro_x","gyro_y","gyro_z",
                "mag_x","mag_y","mag_z"],true);
            telemega_imu_data.name = "Altus TeleMega IMU Sensor Data".to_owned();
            _ = ps_manager.register_packet_structure(&mut telemega_imu_data);

            // TeleMega Kalman Data Packet.
            let mut telemega_kalman_structure = PacketStructure::default();
            telemega_kalman_structure.ez_make("_2 u16 09 u8 i16 i16 i8 i8 i8 i8 i8 i8 i32 i16 i16 i16 i16 i16 i16", 
            &["Timestamp","state",
                "v_batt","v_pyro",
                "sense_1","sense_2","sense_3","sense_4","sense_5","sense_6",
                "ground_pres","ground_accel",
                "accel_plus_g","accel_minus_g",
                "acceleration","speed","height"],true);
            telemega_kalman_structure.name = "Altus TeleMega Kalman and Voltage Data".to_owned();
            _ = ps_manager.register_packet_structure(&mut telemega_kalman_structure);

            //TeleMetrum sensor data packet
            let mut metrum_sensor_data = PacketStructure::default();
            metrum_sensor_data.ez_make("_2 u16 0A u8 i16 i32 i16 i16 i16 i16 i16 i16 i16", 
            &["Timestamp","Flight state",
            "accelerometer",
            "pressure sensor (Pa * 10)","temperature sensor (°C * 100)",
            "acceleration m/s² * 16","speed m/s * 16","height m",
            "battery voltage","drogue continuity sense","main continuity sense"],true);
            metrum_sensor_data.name = "Altus TeleMetrum v2 Sensor Data".to_owned();
            _ = ps_manager.register_packet_structure(&mut metrum_sensor_data);

            //TeleMetrum calibration data
            let mut metrum_calibration_data = PacketStructure::default();
            metrum_calibration_data.ez_make("_2 u16 0B _3 i32 i16 i16 i16", 
            &["Timestamp",
                    "ground_pres","ground_accel",
                    "accel_plus_g","accel_minus_g"],true);
            metrum_calibration_data.name = "Altus TeleMetrum v2 Calibration Data".to_owned();
            _ = ps_manager.register_packet_structure(&mut metrum_calibration_data);

            //AltusMetrum config packet
            let mut altus_config_packet = PacketStructure::default();
            altus_config_packet.ez_make("_2 u16 04 u8 u16 u8 u8 u16 u16 u16 u64 u64", 
            &["Timestamp",
                    "Device type","Flight number",
                    "Config major version","Config minor version",
                    "Apogee deploy delay in seconds","Main deploy alt in meters","Maximum flight log size (kB)",
                    "Radio ID String","Software Version String"],true);
            altus_config_packet.name = "Altus Configuration Data".to_owned();
            _ = ps_manager.register_packet_structure(&mut altus_config_packet);

            //AltusMetrum gps packet
            let mut altus_gps_packet = PacketStructure::default();
            altus_gps_packet.ez_make("_2 u16 05 u8 i16 i32 i32 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u16 i16 u8", 
            &["Timestamp","flags",
                    "altitude (m)",
                    "latitude degrees * 107","longitude degrees * 107",
                    "year","month","day","hour","minute","second",
                    "pdop * 5","hdop * 5","vdop * 5","mode",
                    "ground_speed cm/s", "climb_rate cm/s",
                    "course / 2"],true);
            altus_gps_packet.name = "Altus GPS Location".to_owned();
            _ = ps_manager.register_packet_structure(&mut altus_gps_packet);

            //AltusMetrum satellite packet
            let mut altus_satellite_packet = PacketStructure::default();
            altus_satellite_packet.ez_make("_2 u16 06 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8", 
            &["Timestamp","channels",
                    "sat_1_svid", "sat_1_c_n_1",
                    "sat_2_svid", "sat_2_c_n_1",
                    "sat_3_svid", "sat_3_c_n_1",
                    "sat_4_svid", "sat_4_c_n_1",
                    "sat_5_svid", "sat_5_c_n_1",
                    "sat_6_svid", "sat_6_c_n_1",
                    "sat_7_svid", "sat_7_c_n_1",
                    "sat_8_svid", "sat_8_c_n_1",
                    "sat_9_svid", "sat_9_c_n_1",
                    "sat_10_svid", "sat_10_c_n_1",
                    "sat_11_svid", "sat_11_c_n_1",
                    "sat_12_svid", "sat_12_c_n_1"],true);
            altus_satellite_packet.name = "Altus GPS Satellite Data".to_owned();
            _ = ps_manager.register_packet_structure(&mut altus_satellite_packet);

        });
        return TeleDongleAdapter{
            port: None,
            packet_parser: Default::default(),
            baud: 0,
            id: 0,
            packet_structure_manager: packet_structure_manager
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
            let mut new_port = serialport::new(port_name, self.baud).flow_control(serialport::FlowControl::None).open()?;
            // Short non-zero timeout is needed to receive data from the serialport when
            // the buffer isn't full yet.
            new_port.set_timeout(std::time::Duration::from_millis(1))?;
            self.port = Some(new_port);

            //setup commands for the radio
            self.write_port(&vec![0x7E, 0x0A, 0x45,0x20,0x30,0x0A,0x6D,0x20,0x30,0x0A])?;
            self.get_device_packets(&mut vec![])?;
            self.write_port(&vec![0x6D,0x20,0x32,0x30,0x0A,0x6D,0x20,0x30,0x0A,0x63,0x20,0x73,0x0A,0x66,0x0A,0x76,0x0A])?;
            self.get_device_packets(&mut vec![])?;
            self.write_port(&vec![0x6D,0x20,0x32,0x30,0x0A,0x6D,0x20,0x30,0x0A,0x63,0x20,0x46,0x20,0x34,0x33,0x34,0x35,0x35,0x30,0x0A,0x6D,0x20,0x32,0x30,0x0A,0x6D,0x20,0x30,0x0A,0x6D,0x20,0x32,0x30,0x0A,0x6D,0x20,0x30,0x0A,0x63,0x20,0x54,0x20,0x30,0x0A,0x6D,0x20,0x32,0x30,0x0A,0x6D,0x20,0x32,0x30,0x0A])?;
        }
        Ok(())
    }

    /// Returns true if there is an active port
    fn is_init(&self) -> bool {
        self.port.is_some()
    }

    /// Reads bytes from the active port and adds new bytes to the write_buffer
    /// returns an empty set when the function runs successfully, 
    /// 
    /// # Errors
    /// bails and returns an error if there is no active port
    fn get_device_packets(&mut self, write_buffer: &mut Vec<Packet>) -> anyhow::Result<()> {
        let active_port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No read port has been set")
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
        let decoded = hex::decode(parsed_str)?;
        
        // Clone to a vec so we can return it easily, especially as we don't
        // know how large it will end up being at compile time.
        self.packet_parser.push_data(&decoded, PRINT_PARSING);
        use_state_in_mutex(&self.packet_structure_manager, &mut |ps_manager| -> anyhow::Result<()>{
            write_buffer.extend_from_slice(&self.packet_parser.parse_packets(ps_manager, PRINT_PARSING)?); 
            Ok(())
        }).expect("poison!")?;
        Ok(())
    }

    /// Attempt to write bytes to the radio test port
    /// 
    /// # Errors
    /// 
    /// returns an error if there is no active port
    fn write_port(&mut self, packet: &[u8]) -> anyhow::Result<()> {
        let port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No active test port")
        };

        port.write(packet)?;

        Ok(())
    }

    fn set_id(&mut self, id: usize){
        self.id = id;
    }
    fn get_id(&self) -> usize {
        return self.id;
    }
    
    fn get_type(&self) -> String {
        return "TeleDongle".to_owned();
    }
    
    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        let active_port = match self.port.as_mut() {
            Some(port) => port,
            None => bail!("No read port has been set")
        };

        let mut buffer = [0; 4096];
        let _bytes_read = active_port.read(&mut buffer)?;
        let str = from_utf8(&buffer)?;
        let mut parsed_str = "".to_owned();
        for c in str.chars() {
            if c.is_ascii_hexdigit(){
                parsed_str.push(c);
            }
        }
        data_vector.append(&mut hex::decode(parsed_str)?);
        return Ok(());
    }
    
    fn parse_device_data(&mut self, data_vector: &mut Vec<u8>, packet_vector: &mut Vec<Packet>) -> anyhow::Result<()> {
        self.packet_parser.push_data(&data_vector, PRINT_PARSING);
        use_state_in_mutex(&self.packet_structure_manager, &mut |ps_manager| -> anyhow::Result<()>{
            packet_vector.extend_from_slice(&self.packet_parser.parse_packets(ps_manager, PRINT_PARSING)?); 
            Ok(())
        }).expect("poison!")?;
        return Ok(());
    }
}
