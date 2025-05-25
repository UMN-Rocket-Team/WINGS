use std::{collections::VecDeque, sync::{Arc, Mutex}};

use crate::{models::packet::{Packet, PacketFieldValue}, packet_structure_manager::PacketStructureManager, state::mutex_utils::use_state_in_mutex};

const MOTOR_WEIGHT_CONSERVATIVE_NEWTONS: f64 = 500.0;
/// A `Mutex` wrapper for `DataProcessor`
pub type DataProcessorState = Mutex<DataProcessor>;
#[derive(Default)]
pub struct DataProcessor {
    daq_id: usize,
    daq_adv_id: usize,
    daq_timestamp_buffer: VecDeque<f64>,
    impulse_estimate: f64,
    max_impulse_estimate:f64,
    burning: bool,
    burn_start_time: f64,
    burn_iter: usize,
    max_pressure: f64,
    burn_time: f64
}

impl DataProcessor{
    pub fn default_state ( ps_manager: Arc<Mutex<PacketStructureManager>>) -> DataProcessorState{
        use_state_in_mutex(&ps_manager, &mut |ps_ref| {
            Mutex::new(
                DataProcessor { 
                    daq_id: ps_ref.get_packet_structure_by_name("daq"),
                    daq_adv_id: ps_ref.enforce_packet_fields("daq_adv",vec!["Time","PSI","Newtons","Impulse","Burn_time","Max_pressure"]),
                    daq_timestamp_buffer: VecDeque::new(),
                    impulse_estimate: 0.0,
                    max_impulse_estimate: 0.0,
                    burning: false,
                    burn_start_time: 0.0,
                    burn_iter: 0,
                    max_pressure: 0.0,
                    burn_time: 0.0
                }
            )
        }).expect("poison!")
    }

    pub fn daq_processing(&mut self, input_array: &mut Vec<Packet>){
        let mut output_array = vec![];
        for packet in input_array.clone() {
            if packet.structure_id == self.daq_id && packet.field_data.len() == 5{
                let mut time = packet.field_data[0].clone();
                time.edit_number(&mut |time| {
                    self.daq_timestamp_buffer.push_front(*time); 
                    *time
                });
                let mut load_cell_raw = packet.field_data[1].clone();
                let mut pressure_raw = packet.field_data[2].clone();

                let mut pressure_psi = pressure_raw.new_number(&mut |v| ((*v - 5.0)/4.0) *3000.0);
                let mut load_cell_newtons =load_cell_raw.new_number(&mut |v| (*v * 920.0) +84.3);
                load_cell_newtons.edit_number(&mut |n| {
                    if *n > MOTOR_WEIGHT_CONSERVATIVE_NEWTONS {
                        self.burn_iter+=1;
                        if self.burn_iter >=3 {
                            let t1 = match self.daq_timestamp_buffer.front(){
                                Some(front) => *front,
                                None => 0.0
                            };
                            let t2 = match self.daq_timestamp_buffer.get(1){
                                Some(front) => *front,
                                None => 0.0
                            };
                            if !self.burning{
                                self.burn_start_time = t1
                            }
                            self.impulse_estimate += *n * (t1 - t2);
                        
                            pressure_psi.edit_number(&mut |psi| 
                                {
                                    self.max_pressure = f64::max(*psi,self.max_pressure);
                                    *psi
                                }
                            );

                            self.max_impulse_estimate = f64::max(self.max_impulse_estimate,self.impulse_estimate);

                            self.burn_time = t1-self.burn_start_time;
                            self.burning = true;
                        }
                    }
                    else {
                        self.impulse_estimate = 0.0;
                        self.burn_iter = 0;
                        self.burning = false;
                    }
                    output_array.push(Packet{
                                structure_id: self.daq_adv_id,
                                field_data: vec![
                                    time.clone(),
                                    pressure_psi.clone(),
                                    PacketFieldValue::Number(*n),
                                    PacketFieldValue::Number(self.max_impulse_estimate),
                                    PacketFieldValue::Number(self.burn_time),
                                    PacketFieldValue::Number(self.max_pressure)
                                ],
                            });
                    *n
                });
            }
        }
        input_array.append(&mut output_array);
        
    }
}
