use std::sync::Mutex;

use anyhow::anyhow;
use chrono::Duration;
use tauri::{AppHandle, Manager};
use timer::{Guard, Timer};

use crate::{
    models::packet::Packet, mutex_utils::use_state_in_mutex,
    packet_parser_state::use_packet_parser, packet_parser_state::PacketParserState,
    packet_structure_manager_state::PacketStructureManagerState, serial::SerialPortNames,
    serial_manager_state::SerialManagerState, use_packet_structure_manager, state::serial_manager_state::use_serial_manager
};

pub struct TimerState {
    refresh_timer_data: Mutex<RefreshTimerData>,
}

impl TimerState {
    pub fn new(app_handle: AppHandle) -> Self {
        let timer = Timer::new();

        let update_task_guard = timer.schedule_repeating(Duration::seconds(1), move || {
            match refresh_available_ports_and_read_active_port(
                app_handle.state::<SerialManagerState>(),
                app_handle.state::<PacketStructureManagerState>(),
                app_handle.state::<PacketParserState>(),
            ) {
                Ok(result) => {
                    //sends packets to frontend
                    if result.new_available_port_names != None || result.parsed_packets != None {
                        app_handle.emit_all("serial-update", result).unwrap();
                    }
                }
                Err(message) => app_handle.emit_all("error", message).unwrap(),
            };
        });

        return Self {
            refresh_timer_data: Mutex::new(RefreshTimerData {
                timer,
                update_task_guard,
            }),
        };
    }

    pub fn destroy(&self) {
        match use_state_in_mutex::<RefreshTimerData, (), &str>(
            &self.refresh_timer_data,
            &mut |refresh_timer_data| {
                drop(&refresh_timer_data.update_task_guard);
                drop(&refresh_timer_data.timer);
                Ok(())
            },
        ) {
            _ => {}
        }
    }
}

struct RefreshTimerData {
    timer: Timer,
    update_task_guard: Guard,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct RefreshAndReadResult {
    new_available_port_names: Option<Vec<SerialPortNames>>,
    parsed_packets: Option<Vec<Packet>>,
}

/// Refreshes list of ports available
/// reads from active ports and returns parsed data
fn refresh_available_ports_and_read_active_port(
    serial_manager_state: tauri::State<'_, SerialManagerState>,
    packet_structure_manager_state: tauri::State<'_, PacketStructureManagerState>,
    packet_parser_state: tauri::State<'_, PacketParserState>,
) -> Result<RefreshAndReadResult, String> {
    let mut result: RefreshAndReadResult = RefreshAndReadResult {
        new_available_port_names: None,
        parsed_packets: None,
    };
    let mut read_data: Vec<u8> = vec![];

    match use_serial_manager(serial_manager_state, &mut |serial_manager| {
        let new_ports = serial_manager.get_new_available_ports();
        result.new_available_port_names = new_ports;

        if serial_manager.has_active_port() {
            match serial_manager.read_active_port() {
                Ok(data) => read_data.extend(data),
                Err(error) => return Err(anyhow!(error.to_string()))
            }
        }

        Ok(())
    }) {
        Ok(_) => {}
        Err(message) => return Err(message)
    }

    if !read_data.is_empty() {
        match use_packet_parser(packet_parser_state, &mut |packet_parser| {
            packet_parser.push_data(&read_data);

            use_packet_structure_manager::<(), &str>(
                &packet_structure_manager_state,
                &mut |packet_structure_manager| {
                    Ok(result.parsed_packets =
                        Some(packet_parser.parse_packets(&packet_structure_manager)))
                },
            )
        }) {
            Ok(_) => {}
            Err(message) => return Err(message),
        }
    }

    Ok(result)
}
