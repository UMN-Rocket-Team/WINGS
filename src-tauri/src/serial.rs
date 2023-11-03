use std::{sync::{mpsc, Mutex}, time::Duration, thread, ops::Deref};

use anyhow::bail;
use serde::Serialize;
use tauri::Manager;

const BAUD_RATE: u32 = 57600;

const TEST_MAGIC_BYTE: u8 = 42;
const TEST_PAYLOAD_SIZE: usize = 8;

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

struct StopMessage();

fn should_continue(rx: &mpsc::Receiver<StopMessage>) -> bool {
    match rx.try_recv() {
        // Stop if the parent thread has sent us a stop message or has died
        Ok(_) | Err(mpsc::TryRecvError::Disconnected) => false,
        _ => true,
    }
}

// Inner value is the receiving end of a channel shared with the thread that runs the
// test. To stop the test, send a StopMessage.
pub struct BackgroundTask(mpsc::Sender<StopMessage>);

impl BackgroundTask {
    /// Stops the task.
    pub fn stop(&self) {
        // Error can be ignored as if the channel is already closed, then the
        // thread was already stopped.
        let _ = self.0.send(StopMessage());
    }
}

#[derive(Default)]
pub struct SerialManager {
    previous_available_ports: Vec<SerialPortNames>,
    active_port: Option<Box<dyn serialport::SerialPort>>,
    send_test: Mutex<Option<BackgroundTask>>,
    receive_test: Mutex<Option<BackgroundTask>>
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

        // Send an initial state update so the frontend knows the port was opened successfully
        let _ = app_handle.emit_all("radio-test-send-update", SendingState::default());

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut packets_sent = 0;

            while should_continue(&rx) {
                let _ = port.write(&u8::to_le_bytes(TEST_MAGIC_BYTE));
                for i in 1..TEST_PAYLOAD_SIZE {
                    let _ = port.write(&u8::to_le_bytes(i.try_into().unwrap()));
                }

                packets_sent += 1;
                let _ = app_handle.emit_all("radio-test-send-update", SendingState {
                    packets_sent
                });
                println!("Sent packet {}", packets_sent);

                thread::sleep(interval);
            }
        });

        *guard.unwrap() = Some(BackgroundTask(tx));
        Ok(())
    }

    /// Begin receiving test packets. Events are sent directly to the frontend.
    pub fn start_receive_test(&mut self, app_handle: tauri::AppHandle, port_name: &str) -> anyhow::Result<()> {
        let guard = self.receive_test.lock();
        if !guard.is_ok() {
            bail!("Failed to lock")
        }

        let mut port = serialport::new(port_name, BAUD_RATE).open()?;
        port.clear(serialport::ClearBuffer::All)?;
        port.set_timeout(Duration::from_millis(100))?;

        // Send an initial state update so the frontend knows the port was opened successfully
        let _ = app_handle.emit_all("radio-test-receive-update", ReceivingState::default());

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut packets_read = 0;

            while should_continue(&rx) {
                let mut buffer = [0; TEST_PAYLOAD_SIZE];
                if let Ok(bytes_read) = port.read(&mut buffer) {
                    let read_data = &buffer[0..bytes_read];
                    println!("Read data: {:?}", read_data);
                    for i in read_data.iter() {
                        if *i == TEST_MAGIC_BYTE {
                            packets_read += 1;
                        }
                    }
                }

                let _ = app_handle.emit_all("radio-test-receive-update", ReceivingState {
                    packets_read
                });
            }
        });

        *guard.unwrap() = Some(BackgroundTask(tx));
        Ok(())
    }

    /// Stop any ongoing sending and receiving tests.
    pub fn stop_tests(&mut self) {
        let mut write_test_guard = self.send_test.lock().unwrap();
        match write_test_guard.deref() {
            Some(radio_test) => radio_test.stop(),
            None => {}
        }
        *write_test_guard = None;
        drop(write_test_guard);

        let mut read_test_guard = self.receive_test.lock().unwrap();
        match read_test_guard.deref() {
            Some(radio_test) => radio_test.stop(),
            None => {}
        }
        *read_test_guard = None;
        drop(read_test_guard);
    }
}
