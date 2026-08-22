use serde::{Deserialize, Serialize};
use serialport::{available_ports, SerialPort};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, State};

#[derive(Debug, Serialize, Deserialize)]
pub struct PortInfo {
    pub name: String,
    pub port_type: String,
}

struct SerialConnection {
    baud_rate: u32,
    writer: Box<dyn SerialPort>,
    generation: u64,
}

#[derive(Default)]
struct SerialPortsState {
    connections: HashMap<String, SerialConnection>,
    next_generation: u64,
}

#[derive(Debug, Clone, Serialize)]
struct SerialPortStatusResponse {
    port_name: String,
    baud_rate: u32,
    is_open: bool,
}

#[derive(Clone, Serialize)]
struct SerialBytesEvent {
    port_name: String,
    data: Vec<u8>,
}

#[derive(Clone, Serialize)]
struct SerialTextEvent {
    port_name: String,
    data: String,
}

fn emit_accumulated(app: &AppHandle, port_name: &str, data: &mut Vec<u8>) {
    if data.is_empty() {
        return;
    }

    let message = match String::from_utf8(data.clone()) {
        Ok(message) => message,
        Err(_) => format!(
            "HEX: {}",
            data.iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<_>>()
                .join(" ")
        ),
    };
    app.emit(
        "serial_data",
        SerialTextEvent {
            port_name: port_name.to_string(),
            data: message,
        },
    )
    .ok();
    data.clear();
}

#[tauri::command]
fn open_serial_port(
    app_handle: AppHandle,
    port_name: &str,
    baud_rate: u32,
    state: State<Arc<Mutex<SerialPortsState>>>,
) -> Result<String, String> {
    let writer = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(20))
        .open()
        .map_err(|error| format!("Failed to open port {}: {}", port_name, error))?;
    let mut reader = writer
        .try_clone()
        .map_err(|error| format!("Failed to clone port {}: {}", port_name, error))?;

    let generation = {
        let mut guard = state
            .lock()
            .map_err(|error| format!("Failed to lock state: {}", error))?;
        if guard.connections.contains_key(port_name) {
            return Err(format!("Port {} is already open", port_name));
        }
        guard.next_generation = guard.next_generation.wrapping_add(1);
        let generation = guard.next_generation;
        guard.connections.insert(
            port_name.to_string(),
            SerialConnection {
                baud_rate,
                writer,
                generation,
            },
        );
        generation
    };

    let port_name = port_name.to_string();
    let reader_port_name = port_name.clone();
    let reader_state = Arc::clone(&state);
    let reader_app = app_handle.clone();

    thread::spawn(move || {
        reader_app
            .emit(
                "serial_message",
                SerialTextEvent {
                    port_name: reader_port_name.clone(),
                    data: format!("Port {} opened at {} bps", reader_port_name, baud_rate),
                },
            )
            .ok();

        let mut buffer = [0_u8; 1024];
        let mut accumulated = Vec::new();
        let mut last_send = Instant::now();

        loop {
            let is_current = reader_state
                .lock()
                .map(|guard| {
                    guard
                        .connections
                        .get(&reader_port_name)
                        .is_some_and(|connection| connection.generation == generation)
                })
                .unwrap_or(false);
            if !is_current {
                break;
            }

            match reader.read(&mut buffer) {
                Ok(bytes_read) if bytes_read > 0 => {
                    let bytes = buffer[..bytes_read].to_vec();
                    reader_app
                        .emit(
                            "serial_data_bytes",
                            SerialBytesEvent {
                                port_name: reader_port_name.clone(),
                                data: bytes.clone(),
                            },
                        )
                        .ok();
                    accumulated.extend_from_slice(&bytes);
                }
                Ok(_) => thread::sleep(Duration::from_millis(10)),
                Err(error) if error.kind() == std::io::ErrorKind::TimedOut => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => {
                    reader_app
                        .emit(
                            "serial_error",
                            SerialTextEvent {
                                port_name: reader_port_name.clone(),
                                data: format!("Error reading from port: {}", error),
                            },
                        )
                        .ok();
                    break;
                }
            }

            if accumulated.len() >= 32
                || (accumulated.len() > 2 && last_send.elapsed() > Duration::from_millis(100))
            {
                emit_accumulated(&reader_app, &reader_port_name, &mut accumulated);
                last_send = Instant::now();
            }
        }

        emit_accumulated(&reader_app, &reader_port_name, &mut accumulated);
        let removed_current = if let Ok(mut guard) = reader_state.lock() {
            let should_remove = guard
                .connections
                .get(&reader_port_name)
                .is_some_and(|connection| connection.generation == generation);
            if should_remove {
                guard.connections.remove(&reader_port_name);
            }
            should_remove
        } else {
            false
        };
        if removed_current {
            reader_app
                .emit(
                    "serial_message",
                    SerialTextEvent {
                        port_name: reader_port_name.clone(),
                        data: format!("Port {} closed", reader_port_name),
                    },
                )
                .ok();
            reader_app
                .emit(
                    "serial_status_changed",
                    SerialPortStatusResponse {
                        port_name: reader_port_name,
                        baud_rate,
                        is_open: false,
                    },
                )
                .ok();
        }
    });

    app_handle
        .emit(
            "serial_status_changed",
            SerialPortStatusResponse {
                port_name: port_name.clone(),
                baud_rate,
                is_open: true,
            },
        )
        .ok();
    Ok(format!("Opening port {} at {} bps", port_name, baud_rate))
}

#[tauri::command]
fn close_serial_port(
    app_handle: AppHandle,
    port_name: &str,
    state: State<Arc<Mutex<SerialPortsState>>>,
) -> Result<String, String> {
    let baud_rate = {
        let mut guard = state
            .lock()
            .map_err(|error| format!("Failed to lock state: {}", error))?;
        guard
            .connections
            .remove(port_name)
            .map(|connection| connection.baud_rate)
            .ok_or_else(|| format!("Port {} is not open", port_name))?
    };
    app_handle
        .emit(
            "serial_message",
            SerialTextEvent {
                port_name: port_name.to_string(),
                data: format!("Port {} closed", port_name),
            },
        )
        .ok();
    app_handle
        .emit(
            "serial_status_changed",
            SerialPortStatusResponse {
                port_name: port_name.to_string(),
                baud_rate,
                is_open: false,
            },
        )
        .ok();
    Ok(format!("Closing port {}", port_name))
}

#[tauri::command]
fn send_to_serial_port(
    port_name: &str,
    data: Vec<u8>,
    state: State<Arc<Mutex<SerialPortsState>>>,
) -> Result<usize, String> {
    if data.is_empty() {
        return Ok(0);
    }

    let mut guard = state
        .lock()
        .map_err(|error| format!("Failed to lock state: {}", error))?;
    let connection = guard
        .connections
        .get_mut(port_name)
        .ok_or_else(|| format!("Port {} is not open", port_name))?;
    connection
        .writer
        .write_all(&data)
        .and_then(|_| connection.writer.flush())
        .map_err(|error| format!("Failed to write to {}: {}", port_name, error))?;
    Ok(data.len())
}

#[tauri::command]
fn get_available_ports() -> Result<Vec<PortInfo>, String> {
    available_ports()
        .map(|ports| {
            ports
                .into_iter()
                .map(|port| PortInfo {
                    name: port.port_name,
                    port_type: match port.port_type {
                        serialport::SerialPortType::UsbPort(info) => info
                            .product
                            .unwrap_or_else(|| "USB serial device".to_string()),
                        other => format!("{:?}", other),
                    },
                })
                .collect()
        })
        .map_err(|error| format!("Failed to list serial ports: {}", error))
}

#[tauri::command]
fn get_serial_port_status(
    state: State<Arc<Mutex<SerialPortsState>>>,
) -> Result<Vec<SerialPortStatusResponse>, String> {
    let guard = state
        .lock()
        .map_err(|error| format!("Failed to lock state: {}", error))?;
    Ok(guard
        .connections
        .iter()
        .map(|(port_name, connection)| SerialPortStatusResponse {
            port_name: port_name.clone(),
            baud_rate: connection.baud_rate,
            is_open: true,
        })
        .collect())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Arc::new(Mutex::new(SerialPortsState::default())))
        .invoke_handler(tauri::generate_handler![
            get_available_ports,
            open_serial_port,
            close_serial_port,
            send_to_serial_port,
            get_serial_port_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
