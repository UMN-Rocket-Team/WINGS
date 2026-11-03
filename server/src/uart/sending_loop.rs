use std::{
    sync::{Mutex, mpsc},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use csv::StringRecord;

use wings_packet_parser::prelude::*;

use crate::APP_STATE;

/// Name of the event sent to the frontend.
// const SENDING_LOOP_UPDATE: &str = "sending-loop-update";

/// A stoppable activity running in a background thread.
pub struct BackgroundTask {
    // Receiving end of a channel shared with the thread running the task.
    // When the BackgroundTask struct is dropped, this will be dropped too,
    // which the child thread can detect.
    _stop_send: mpsc::Sender<()>,
}

impl BackgroundTask {
    /// Repeatedly run the given closure in a background thread until the
    /// returned struct is dropped.
    ///
    /// The callback just should do something once -- it shouldn't include
    /// its own `loop { ... }` as that's handled by BackgroundTask.
    pub fn run_repeatedly<F>(mut callback: F) -> BackgroundTask
    where
        F: AsyncFnMut(),
        F: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let is_stopped = || matches!(rx.try_recv(), Err(mpsc::TryRecvError::Disconnected));

            while !is_stopped() {
                let _ = callback();
            }
        });

        BackgroundTask { _stop_send: tx }
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

/// A `Mutex` wrapper for `SendingLoop`
pub type SendingLoopState = Mutex<SendingLoop>;
#[derive(Default)]
pub struct SendingLoop {
    task: Option<BackgroundTask>,
}

impl SendingLoop {
    pub async fn start(
        &mut self,
        interval: Duration,
        already_sent: u32,
        mode: SendingModes,
        write_id: usize,
    ) -> anyhow::Result<()> {
        let mut communication_manager = APP_STATE.wait().communication_manager().await;
        let packet_structure = communication_manager.ps_manager.lock().unwrap().packet_structures()[1].clone();

        // Send an initial state update so the frontend knows the port was opened successfully
        // let _ = app_handle.emit(SENDING_LOOP_UPDATE, SendingState::starting());
        // Sleeping always needs to happen at the end of the task, even if we return early.
        let sleep = move || {
            thread::sleep(interval);
        };

        let mut flipper: u8 = 0;
        let mut packets_sent: u32 = already_sent;

        self.task = Some(BackgroundTask::run_repeatedly(async move || {
            let file_handler = &mut APP_STATE.wait().log_handler().await;
            let packet_to_send: Option<StringRecord>;

            // ##########################
            // Get the packet string record Depending on mode
            // ##########################
            match mode {
                SendingModes::FromCSV => match file_handler.read_packet() {
                    Ok(packet) => {
                        packet_to_send = Some(packet);
                    }
                    Err(err) => {
                        println!("Failed to lock file handler: {}", err);
                        sleep();
                        return;
                    }
                },
                SendingModes::AllZeros => {
                    let mut output_string = vec![];
                    for _i in 0..packet_structure.fields.len() {
                        output_string.push("0");
                    }
                    packet_to_send = Some(StringRecord::from(output_string));
                }
                SendingModes::AllOnes => {
                    let mut output_string = vec![];
                    for _i in 0..packet_structure.fields.len() {
                        output_string.push("1");
                    }
                    packet_to_send = Some(StringRecord::from(output_string));
                }
                SendingModes::Alternating => {
                    flipper = !flipper;
                    let mut output_string = vec![unix_time().to_string()];
                    for i in 1..packet_structure.fields.len() {
                        output_string.push((flipper.overflowing_add(i as u8).0).to_string());
                    }
                    packet_to_send = Some(StringRecord::from(output_string));
                }
                SendingModes::TimeStampAndIncreasing => {
                    let mut output_string = vec![unix_time().to_string()];
                    for i in 1..packet_structure.fields.len() {
                        output_string
                            .push(((packets_sent.overflowing_add(i as u32).0) as u8).to_string());
                    }
                    packet_to_send = Some(StringRecord::from(output_string));
                }
            }

            // ##########################
            // Generate Packet
            // ##########################
            let packet = match generate_packet(&packet_structure, packet_to_send.unwrap()) {
                Ok(packet) => packet,
                Err(err) => {
                    println!("Failed to generate test packet: {}", err);
                    sleep();
                    return;
                }
            };

            // ##########################
            // Send Packet
            // ##########################
            match communication_manager.write_data(&packet, write_id) {
                Ok(_) => {
                    packets_sent = packets_sent.overflowing_add(1).0;
                    //println!("Sent packet {}: {:?}", packets_sent, packet);

                    // let _ =
                    //     app_handle.emit(SENDING_LOOP_UPDATE, SendingState::sent(packets_sent));
                }
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
