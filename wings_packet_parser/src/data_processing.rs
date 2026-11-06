//! # Real-time Data Processing Engine
//!
//! This module is responsible for processing raw data packets as they are received.

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{
    models::packet::{Packet, PacketFieldValue},
    packet_structure_manager::PacketStructureManager,
    state::mutex_utils::use_state_in_mutex,
};

/// A conservative estimate for the weight of the motor in Newtons.
/// This is used as a threshold to detect when the motor is under thrust.
const MOTOR_WEIGHT_CONSERVATIVE_NEWTONS: f64 = 500.0;
/// A `Mutex` wrapper for `DataProcessor`
pub type DataProcessorState = Mutex<DataProcessor>;

/// Holds the state and logic for processing incoming data packets.
///
/// This struct is stateful, meaning it retains information between calls to its processing methods.
/// This is essential for tasks like calculating impulse over time or detecting the start and end of a motor burn.
#[derive(Default)]
pub struct DataProcessor {
    daq_id: usize,
    daq_adv_id: usize,
    altus_sense_id: usize,
    altus_gps_id: usize,
    daq_timestamp_buffer: VecDeque<f64>,
    impulse_estimate: f64,
    max_impulse_estimate: f64,
    burning: bool,
    burn_start_time: f64,
    burn_iter: usize,
    max_pressure: f64,
    burn_time: f64,
}

impl DataProcessor {
    /// Creates a new `DataProcessorState` with default values.
    ///
    /// This constructor initializes the processor by querying the `PacketStructureManager`
    /// to get the unique IDs for the packet types it needs to process. It also ensures
    /// that the packet structure for its derived data ("daq_adv") exists and has the correct fields.
    ///
    /// # Arguments
    /// * `ps_manager` - A thread-safe handle to the `PacketStructureManager`.
    pub fn default_state(ps_manager: Arc<Mutex<PacketStructureManager>>) -> DataProcessorState {
        use_state_in_mutex(&ps_manager, &mut |ps_ref| {
            Mutex::new(DataProcessor {
                daq_id: ps_ref.get_packet_structure_by_name("daq"),
                // Ensures the derived data packet exists with the necessary fields.
                daq_adv_id: ps_ref.enforce_packet_fields(
                    "daq_adv",
                    vec![
                        "Time",
                        "PSI",
                        "Newtons",
                        "Impulse",
                        "Burn_time",
                        "Max_pressure",
                    ],
                ),
                altus_sense_id: ps_ref
                    .get_packet_structure_by_name("Altus TeleMega Kalman and Voltage Data"),
                altus_gps_id: ps_ref.get_packet_structure_by_name("Altus GPS Location"),
                daq_timestamp_buffer: VecDeque::new(),
                impulse_estimate: 0.0,
                max_impulse_estimate: 0.0,
                burning: false,
                burn_start_time: 0.0,
                burn_iter: 0,
                max_pressure: 0.0,
                burn_time: 0.0,
            })
        })
    }

    /// Processes a vector of incoming packets, performing conversions and deriving new data.
    ///
    /// This is the core method of the `DataProcessor`. It iterates through a batch of packets
    /// and applies specific logic based on the packet's type (identified by its `structure_id`).
    ///
    /// # Arguments
    /// * `input_array` - A mutable vector of `Packet`s to be processed. Some packets may be modified in-place.
    pub fn daq_processing(&mut self, input_array: &mut Vec<Packet>) {
        let mut output_array = vec![];
        for packet in input_array {
            // --- Altus TeleMega Kalman and Voltage Data Processing ---
            if packet.structure_id == self.altus_sense_id && packet.field_data.len() >= 17 {
                // Perform unit conversions on altitude and velocity fields.
                packet.field_data[15].edit_number(&mut |n| *n / (16.0 * 3.28084));
                packet.field_data[16].edit_number(&mut |n| *n / (3.28084));
            }
            // --- Altus GPS Location Processing ---
            if packet.structure_id == self.altus_gps_id && packet.field_data.len() >= 5 {
                println!("here");
                packet.field_data[3].edit_number(&mut |n| *n / (100.0 * 100000.0));
                packet.field_data[4].edit_number(&mut |n| *n / (100.0 * 100000.0));
            }
            // --- DAQ (Data Acquisition) System Processing ---
            if packet.structure_id == self.daq_id && packet.field_data.len() == 5 {
                let mut time = packet.field_data[0].clone();
                time.edit_number(&mut |time| {
                    self.daq_timestamp_buffer.push_front(*time);
                    *time
                });
                let mut load_cell_raw = packet.field_data[1].clone();
                let mut pressure_raw = packet.field_data[2].clone();

                // Convert raw pressure sensor values to PSI.
                let mut pressure_psi =
                    pressure_raw.new_number(&mut |v| ((*v - 5.0) / 4.0) * 3000.0);
                // Convert raw load cell values to Newtons of thrust.
                let mut load_cell_newtons = load_cell_raw.new_number(&mut |v| (*v * 920.0) + 84.3);

                // Main burn detection and impulse calculation logic.
                load_cell_newtons.edit_number(&mut |n| {
                    if *n > MOTOR_WEIGHT_CONSERVATIVE_NEWTONS {
                        // Thrust is above the threshold, indicating a potential burn.
                        self.burn_iter += 1;
                        if self.burn_iter >= 3 {
                            let t1 = match self.daq_timestamp_buffer.front() {
                                Some(front) => *front,
                                None => 0.0,
                            };
                            let t2 = match self.daq_timestamp_buffer.get(1) {
                                Some(front) => *front,
                                None => 0.0,
                            };
                            if !self.burning {
                                self.burn_start_time = t1
                            }
                            // Integrate impulse: Thrust * dt
                            self.impulse_estimate += *n * (t1 - t2);

                            pressure_psi.edit_number(&mut |psi| {
                                self.max_pressure = f64::max(*psi, self.max_pressure);
                                *psi
                            });

                            self.max_impulse_estimate =
                                f64::max(self.max_impulse_estimate, self.impulse_estimate);

                            self.burn_time = t1 - self.burn_start_time;
                            self.burning = true;
                        }
                    } else {
                        // Thrust is below threshold; reset burn state.
                        self.impulse_estimate = 0.0;
                        self.burn_iter = 0;
                        self.burning = false;
                    }

                    // Create and push a new, derived packet with the calculated data.
                    output_array.push(Packet {
                        structure_id: self.daq_adv_id,
                        field_data: vec![
                            time.clone(),
                            pressure_psi.clone(),
                            PacketFieldValue::Number(*n),
                            PacketFieldValue::Number(self.max_impulse_estimate),
                            PacketFieldValue::Number(self.burn_time),
                            PacketFieldValue::Number(self.max_pressure),
                        ],
                    });
                    *n
                });
            }
        }
        //input_array.append(&mut output_array);
    }
}
