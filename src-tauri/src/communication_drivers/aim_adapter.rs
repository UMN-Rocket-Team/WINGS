use std::{
    ffi::CString,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use anyhow::bail;
use hidapi::{HidApi, HidDevice};

use crate::{
    communication_manager::CommsIF, models::packet_parser::PacketParser,
    packet_structure_manager::PacketStructureManager, state::mutex_utils::use_state_in_mutex,
};

const META: &str = "Aim_Meta";
const ACCEL_Z: &str = "Aim_AccelZ";
const PRESSURE: &str = "Aim_Pressure";
const COMP_BATT: &str = "Aim_BatComp";
const EJECT_BATT: &str = "Aim_BatEject";
const TEMP: &str = "Aim_Temp";
const LINE_A: &str = "Aim_LineA";
const LINE_B: &str = "Aim_LineB";
const LINE_C: &str = "Aim_LineC";
const LINE_D: &str = "Aim_LineD";
const ACCEL_XY: &str = "Aim_AccelXY";
const GYRO: &str = "Aim_GyroXYZ";
const MAG: &str = "Aim_MagXYZ";
const GPS: &str = "Aim_GPSLLSOL";
const RSSI: &str = "Aim_RSSI";
const STATUS: &str = "Aim_Status";
const IDENTIFIER: &str = "Aim_Ident";
const GPS_TIME: &str = "Aim_GPSTime";
const TIMESTAMP: &str = "Aim_TimeStamp";
const ORIENTATION: &str = "Aim_Orientation";

pub struct AimAdapter {
    device: Option<HidDevice>,
    packet_parser: Option<Box<dyn PacketParser>>,
    baud: u32,
    id: usize,
    last_read: [u8; 64],
    packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
}

pub fn register_aim_packet_structures(
    ps_manager: &mut PacketStructureManager,
) -> anyhow::Result<()> {
    ps_manager.enforce_packet_fields(META, vec!["System time", "RSSI", "SNR"]);
    ps_manager.enforce_packet_fields(ACCEL_Z, vec!["System time", "Delta time", "Z acceleration"]);
    ps_manager.enforce_packet_fields(PRESSURE, vec!["System time", "Delta time", "Pressure(Pa)"]);
    ps_manager.enforce_packet_fields(COMP_BATT, vec!["System time", "Delta time", "ADC(V)"]);
    ps_manager.enforce_packet_fields(EJECT_BATT, vec!["System time", "Delta time", "ADC(V)"]);
    ps_manager.enforce_packet_fields(TEMP, vec!["System time", "Delta time", "Temperature"]);
    ps_manager.enforce_packet_fields(
        LINE_A,
        vec!["System time", "Delta time", "ADC", "Is_On", "Is_Input"],
    );
    ps_manager.enforce_packet_fields(
        LINE_B,
        vec!["System time", "Delta time", "ADC", "Is_On", "Is_Input"],
    );
    ps_manager.enforce_packet_fields(
        LINE_C,
        vec!["System time", "Delta time", "ADC", "Is_On", "Is_Input"],
    );
    ps_manager.enforce_packet_fields(
        LINE_D,
        vec!["System time", "Delta time", "ADC", "Is_On", "Is_Input"],
    );
    ps_manager.enforce_packet_fields(
        ACCEL_XY,
        vec![
            "System time",
            "Delta time",
            "X acceleration",
            "Y acceleration",
        ],
    );
    ps_manager.enforce_packet_fields(
        GYRO,
        vec![
            "System time",
            "Delta time",
            "X rotation",
            "Y rotation",
            "Z rotation",
        ],
    );
    ps_manager.enforce_packet_fields(
        MAG,
        vec!["System time", "Delta time", "X flux", "Y flux", "Z flux"],
    );
    ps_manager.enforce_packet_fields(
        GPS,
        vec![
            "System time",
            "Delta time",
            "Lat",
            "Long",
            "MSL(mm)",
            "lock",
            "sat_num",
        ],
    );
    ps_manager.enforce_packet_fields(RSSI, vec!["System time", "Delta time", "RSSI"]);
    ps_manager.enforce_packet_fields(
        STATUS,
        vec![
            "System time",
            "Delta time",
            "State",
            "Line D on",
            "Line C on",
            "Line B on",
            "Line A on",
            "Line A continuity",
            "Line B continuity",
            "Line C continuity",
            "Line D continuity",
            "Line A input",
            "Line B input",
            "Line C input",
            "Line D input",
        ],
    );
    ps_manager.enforce_packet_fields(IDENTIFIER, vec!["System time", "Delta time", "Identifier"]);
    ps_manager.enforce_packet_fields(
        GPS_TIME,
        vec![
            "System time",
            "Delta time",
            "iTOW",
            "GPS Week",
            "Valid time",
            "Valid leap secs",
            "leap secs",
        ],
    );
    ps_manager.enforce_packet_fields(TIMESTAMP, vec!["System time", "Delta time", "Timestamp"]);
    ps_manager.enforce_packet_fields(
        ORIENTATION,
        vec![
            "System time",
            "Delta time",
            "Quat x",
            "Quat y",
            "Quat z",
            "Quat w",
        ],
    );

    Ok(())
}

impl CommsIF for AimAdapter {
    ///creates a new instance of a comms device with the given packet structure manager
    fn new(
        packet_structure_manager: Arc<Mutex<PacketStructureManager>>,
        packet_parser: Option<impl PacketParser + 'static>,
    ) -> Self
    where
        Self: Sized,
    {
        use_state_in_mutex(&packet_structure_manager, &mut |ps_manager| {
            if let Err(err) = register_aim_packet_structures(ps_manager) {
                eprintln!("Failed to register Aim packet structures: {err}");
            }
        });
        AimAdapter {
            device: None,
            packet_parser: Some(Box::new(packet_parser.unwrap())),
            baud: 0,
            id: 0,
            packet_structure_manager,
            last_read: [0; 64],
        }
    }

    /// used to connect the object with a specific device
    fn init_device(&mut self, port_name: &str, baud: u32) -> anyhow::Result<()> {
        if port_name.is_empty() {
            self.device = None;
        } else {
            self.baud = baud;
            let hid_api = HidApi::new()?;
            self.device = hid_api.open_path(CString::new(port_name)?.as_c_str()).ok();

            let mut output: [u8; 64] = [0; 64];
            let mut input: [u8; 64] = [0; 64];
            output[0] = 3;
            output[1] = 3;

            match &self.device {
                Some(base_station) => {
                    let _ = base_station.write(&output);
                    sleep(Duration::from_millis(100));
                    let result = base_station.read(&mut input);
                    match result {
                        Ok(_) => {}
                        Err(error) => {
                            println!("{}", error);
                            bail!(anyhow::anyhow!(error)
                                .context("failed to connect to Entacore product"))
                        }
                    }
                }
                None => bail!("no device stored within output of HID API"),
            }
        }
        Ok(())
    }

    /// Attempt to write bytes to the radio test port
    ///
    /// # Errors
    ///
    /// returns an error if the device isn't initialized
    fn write_port(&mut self, _: &[u8]) -> anyhow::Result<()> {
        Err(anyhow::anyhow!(
            "Wings does not currently support sending packets to an Aim-Xtra"
        ))
    }

    /// Returns true if there is an active port
    fn is_init(&self) -> bool {
        self.device.is_some()
    }
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_type(&self) -> String {
        "AimXtra".to_owned()
    }

    fn get_device_raw_data(&mut self, data_vector: &mut Vec<u8>) -> anyhow::Result<()> {
        // let active_port = match self.device.as_mut() {
        //     Some(port) => port,
        //     None => bail!("No read port has been set")
        // };

        // let mut buffer = [0; 4096];
        // let bytes_read = active_port.read(&mut buffer)?;
        // data_vector.extend_from_slice(&buffer[..bytes_read]);
        match &self.device {
            Some(base_station) => {
                let mut output: [u8; 64] = [0; 64];
                let mut input: [u8; 64] = [0; 64];
                output[0] = 0x03;
                output[1] = 0x12;
                let _ = base_station.write(&output);
                let result = base_station.read_timeout(&mut input, 10);
                match result {
                    Ok(_) => {
                        if self.last_read != input {
                            data_vector.extend_from_slice(&input);
                            self.last_read = input;
                        }
                    }
                    Err(_) => {
                        //doing nothing because we didn't read a packet
                    }
                }
                sleep(Duration::from_secs(1));
            }
            None => bail!("not initialized"),
        }
        Ok(())
    }

    fn get_parser(&mut self) -> Option<&mut (dyn PacketParser + 'static)> {
        self.packet_parser.as_deref_mut()
    }

    fn get_packet_structure_manager(&self) -> Arc<Mutex<PacketStructureManager>> {
        self.packet_structure_manager.clone()
    }
}
