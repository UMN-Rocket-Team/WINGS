use std::{time::{Duration, SystemTime, UNIX_EPOCH}, sync::mpsc, thread};
use serde::Serialize;
use tauri::Manager;

use crate::{state::{packet_structure_manager_state::PacketStructureManagerState, communication_state::{use_communication_manager, CommunicationManagerState}}, packet_generator::generate_packet, models::packet::PacketFieldValue};

/// Name of the event sent to the frontend.
const SENDING_LOOP_UPDATE: &str = "sending-loop-update";

/// The object sent to the frontend.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct SendingState {
    packets_sent: u32
}

impl SendingState {
    fn starting() -> Self {
        Self {
            packets_sent: 0
        }
    }

    fn sent(packets_sent: u32) -> Self {
        Self {
            packets_sent
        }
    }
}

/// A stoppable activity running in a background thread.
pub struct BackgroundTask {
    // Receiving end of a channel shared with the thread running the task.
    // When the BackgroundTask struct is dropped, this will be dropped too,
    // which the child thread can detect.
    _stop_send: mpsc::Sender<()>
}

impl BackgroundTask {
    /// Repeatedly run the given closure in a background thread until the
    /// returned struct is dropped.
    /// 
    /// The callback just should do something once -- it shouldn't include
    /// its own `loop { ... }` as that's handled by BackgroundTask.
    pub fn run_repeatedly<F>(mut callback: F) -> BackgroundTask where F: FnMut() -> (), F: Send + 'static {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let is_stopped = || {
                match rx.try_recv() {
                    Err(mpsc::TryRecvError::Disconnected) => true,
                    _ => false
                }
            };

            while !is_stopped() {
                callback();
            }
        });

        BackgroundTask {
            _stop_send: tx
        }
    }
}

fn unix_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_millis()
        .try_into()
        .unwrap_or(i64::MAX)
}

#[derive(Default)]
pub struct SendingLoop {
    task: Option<BackgroundTask>
}

impl SendingLoop {
    pub fn start(&mut self, app_handle: tauri::AppHandle, interval: Duration) -> anyhow::Result<()> {
        let structure_manager_state = app_handle.state::<PacketStructureManagerState>();
        let packet_structure = structure_manager_state.packet_structure_manager.lock().unwrap().packet_structures[0].clone();

        // Send an initial state update so the frontend knows the port was opened successfully
        let _ = app_handle.emit_all(SENDING_LOOP_UPDATE, SendingState::starting());

        // Sleeping always needs to happen at the end of the task, even if we return early.
        let sleep = move || {
            thread::sleep(interval);
        };

        let mut flipper: u8 = 1;
        // let mut millis_to_send: f64 = 10.0;
        let mut packets_sent: u32 = 0;
        self.task = Some(BackgroundTask::run_repeatedly(move || {
            let current_time = unix_time();

            flipper = !flipper;

            let packet = match generate_packet(&packet_structure, &vec![
                PacketFieldValue::SignedLong(current_time),
                PacketFieldValue::UnsignedShort(packets_sent as u16),
                PacketFieldValue::UnsignedShort(packets_sent as u16),
                PacketFieldValue::UnsignedByte(flipper),
                PacketFieldValue::UnsignedByte(!flipper),
                PacketFieldValue::UnsignedShort(0),
                // PacketFieldValue::SignedLong(((millis_to_send.round() as i64)* 1000) + 10000),
            ]) {
                Ok(packet) => packet,
                Err(err) => {
                    println!("Failed to generate test packet: {}", err);
                    sleep();
                    return;
                }
            };
            println!("{:#?}", packet);
            match use_communication_manager(app_handle.state::<CommunicationManagerState>(), &mut |communication_manager| {
                communication_manager.write_data(&packet)
            }) {
                Ok(_) => {
                    packets_sent = packets_sent.wrapping_add(1);
                    //println!("Sent packet {}: {:?}", packets_sent, packet);

                    let _ = app_handle.emit_all(SENDING_LOOP_UPDATE, SendingState::sent(packets_sent));
                },
                Err(err) => {
                    println!("Failed to write to test port: {}", err);
                }
            }

            sleep();
        }));

        Ok(())
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        // BackgroundTask stopping is handled by standard rust lifetimes
        self.task = None;
        Ok(())
    }
}
