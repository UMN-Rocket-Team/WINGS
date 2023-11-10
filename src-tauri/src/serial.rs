use std::{sync::{mpsc, Mutex}, time::{Duration, SystemTime, UNIX_EPOCH}, thread};

use anyhow::bail;
use serde::Serialize;
use tauri::Manager;

use crate::{state::packet_structure_manager_state::PacketStructureManagerState, packet_generator::generate_packet, models::packet::PacketFieldValue};

const BAUD_RATE: u32 = 57600;

#[derive(PartialEq, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerialPortNames {
    name: String,
    manufacturer_name: Option<String>,
    product_name: Option<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct SendingState {
    packets_sent: u32
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct ReceivingState {
    packets_read: u32
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

#[derive(Default)]
pub struct SerialManager {
    previous_available_ports: Vec<SerialPortNames>,
    active_port: Option<Box<dyn serialport::SerialPort>>,
    send_test: Mutex<Option<BackgroundTask>>
}

impl SerialManager {
    /// Returns a list of all possible serial ports
    pub fn get_available_ports(&self) -> Result<Vec<SerialPortNames>, serialport::Error> {
        let ports = serialport::available_ports()?
            .into_iter()
            .filter_map(|port| match port.port_type {
                serialport::SerialPortType::UsbPort(usb_info) => {
                    // On macOS, each serial port shows up as both eg.:
                    //  - /dev/cu.usbserial-AK06O4AO
                    //  - /dev/tty.usbserial-AK06O4AO
                    // For our use, these are equivalent, so we'll filter one out to avoid confusion.
                    if cfg!(target_os = "macos") && port.port_name.starts_with("/dev/cu.usbserial-") {
                        None
                    } else {
                        Some(SerialPortNames {
                            name: port.port_name,
                            manufacturer_name: usb_info.manufacturer,
                            product_name: usb_info.product,
                        })
                    }
                },
                serialport::SerialPortType::PciPort
                | serialport::SerialPortType::BluetoothPort
                | serialport::SerialPortType::Unknown => None,
            })
            .collect();
        Ok(ports)
    }

    /// Return Some() if the ports have changed since the last call, otherwise None if they are the same.
    pub fn get_new_available_ports(&mut self) -> Option<Vec<SerialPortNames>> {
        match self.get_available_ports() {
            Ok(new_ports) => {
                if new_ports == self.previous_available_ports {
                    None
                } else {
                    self.previous_available_ports = new_ports.clone();
                    Some(new_ports)
                }
            },
            Err(_) => None
        }
    }

    /// Set the path of the active port
    /// If path is empty, active port is reset
    pub fn set_active_port(&mut self, port_name: &str) -> anyhow::Result<()> {
        if port_name.is_empty() {
            self.active_port = None;
        } else {
            self.active_port = Some(serialport::new(port_name, BAUD_RATE).open()?);
        }
        Ok(())
    }

    /// Returns true if there is an active port
    pub fn has_active_port(&self) -> bool {
        return self.active_port.is_some()
    }

    /// Read bytes from the active port
    pub fn read_active_port(&mut self) -> anyhow::Result<Vec<u8>> {
        let active_port = match self.active_port.as_mut() {
            Some(port) => port,
            None => bail!("No active port")
        };

        let mut buffer = [0; 1024];
        let bytes_read = active_port.read(&mut buffer)?;

        // Clone to a vec so we can return it easily, especially as we don't
        // know how large it will end up being at compile time.
        let output = buffer[..bytes_read].to_vec();
        Ok(output)
    }

    /// Begin sending test packets. Events are sent directly to the frontend.
    pub fn start_send_test(&mut self, app_handle: tauri::AppHandle, port_name: &str, interval: Duration) -> anyhow::Result<()> {
        let guard = self.send_test.lock();
        if !guard.is_ok() {
            bail!("Failed to lock");
        }

        let mut port = serialport::new(port_name, BAUD_RATE).open()?;
        port.clear(serialport::ClearBuffer::All)?;

        let structure_manager_state = app_handle.state::<PacketStructureManagerState>();
        let packet_structure = structure_manager_state.radio_test_structure.clone();

        // Send an initial state update so the frontend knows the port was opened successfully
        let _ = app_handle.emit_all("radio-test-send-update", SendingState::default());

        let mut packets_sent = 0;
        *guard.unwrap() = Some(BackgroundTask::run_repeatedly(move || {
            // Sleep at start as this can return early in case of errors, but we always
            // want the sleep to happen.
            thread::sleep(interval);

            let unix_millis = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::ZERO)
                .as_millis();

            let packet = match generate_packet(&packet_structure, &vec![
                PacketFieldValue::SignedLong(unix_millis.try_into().unwrap_or(i64::MAX)),
                PacketFieldValue::UnsignedInteger(packets_sent)
            ]) {
                Ok(packet) => packet,
                Err(err) => {
                    println!("Failed to generate test packet: {}", err);
                    return;
                }
            };

            match port.write(&packet) {
                Ok(_) => {
                    packets_sent = packets_sent.wrapping_add(1);
                    println!("Sent packet {}: {:?}", packets_sent, packet);

                    let _ = app_handle.emit_all("radio-test-send-update", SendingState {
                        packets_sent
                    });
                },
                Err(err) => {
                    println!("Failed to write to test port: {}", err);
                }
            }
        }));

        Ok(())
    }

    /// Stop any ongoing sending and receiving tests.
    pub fn stop_send_test(&mut self) {
        // BackgroundTask stopping is handled by standard rust lifetimes

        let mut send_test_guard = self.send_test.lock().unwrap();
        *send_test_guard = None;
        drop(send_test_guard);
    }
}
