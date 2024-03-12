use std::{fs, sync::mpsc, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

use serde::Serialize;
use tauri::Manager;

use crate::{packet_generator::generate_packet, state::{communication_state::{use_communication_manager, CommunicationManagerState}, file_handling_state::{use_file_handler, FileHandlingState}, packet_structure_manager_state::PacketStructureManagerState}};

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

        let mut flipper: bool = true;
        // let mut millis_to_send: f64 = 10.0;
        let mut packets_sent: u16 = 0;
        self.task = Some(BackgroundTask::run_repeatedly(move || {
            let file_handling_state = app_handle.state::<FileHandlingState>();

            flipper = !flipper;
            let packet_to_send;
            match use_file_handler(&file_handling_state, &mut |file_handler|{
                match file_handler.read_packet(){
                    Ok(packet) =>  Ok(packet),
                    Err(err) => Err(err),
                }
            }){
                Ok(packet) => {packet_to_send = Some(packet);},
                Err(_) => return,
            };

            let packet = match generate_packet(&packet_structure, packet_to_send.unwrap())
            {
                Ok(packet) => packet,
                Err(err) => {
                    println!("Failed to generate test packet: {}", err);
                    sleep();
                    return
                }
            };
            //println!("{:#?}", packet);
            match use_communication_manager(app_handle.state::<CommunicationManagerState>(), &mut |communication_manager| {
                communication_manager.write_data(&packet, 1)
            }) {
                Ok(_) => {
                    packets_sent = packets_sent + 1;
                    //println!("Sent packet {}: {:?}", packets_sent, packet);

                    let _ = app_handle.emit_all(SENDING_LOOP_UPDATE, SendingState::sent(packets_sent as u32));
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
