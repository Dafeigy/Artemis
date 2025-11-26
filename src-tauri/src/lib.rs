// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![allow(unused_imports)]
use serialport::{available_ports, SerialPort};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Manager, Emitter, State};
use std::sync::Mutex;

// COM端口信息结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct PortInfo {
    pub name: String,
    pub port_type: String,
}

// 串口连接状态
#[allow(dead_code)]
struct SerialPortState {
    is_open: bool,
    port_name: Option<String>,
    baud_rate: Option<u32>,
    port: Option<Box<dyn SerialPort>>,
}

// 实现默认值
impl Default for SerialPortState {
    fn default() -> Self {
        Self {
            is_open: false,
            port_name: None,
            baud_rate: None,
            port: None,
        }
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 获取可用的COM端口列表
#[tauri::command]
fn open_serial_port(
    app_handle: AppHandle,
    port_name: &str,
    baud_rate: u32,
    state: State<Arc<Mutex<SerialPortState>>>,
) -> Result<String, String> {
    // 尝试关闭已经打开的端口
    let mut state_guard = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    if state_guard.is_open {
        return Err(format!("Port {} is already open", state_guard.port_name.as_ref().unwrap_or(&"unknown".to_string())));
    }
    
    // 更新状态
    state_guard.is_open = true;
    state_guard.port_name = Some(port_name.to_string());
    state_guard.baud_rate = Some(baud_rate);
    
    // 克隆必要的变量以便在新线程中使用
    let app_handle_clone = app_handle.clone();
    let port_name_clone = port_name.to_string();
    let state_clone = Arc::clone(&state);
    
    // 创建一个新线程来读取串口数据
    thread::spawn(move || {
        // 打开串口
        let mut port = match serialport::new(&port_name_clone, baud_rate)
            .timeout(Duration::from_millis(100)) // 设置100ms的读取超时
            .open() {
            Ok(port) => port,
            Err(e) => {
                // 更新状态并发送错误事件
                if let Ok(mut state_guard) = state_clone.lock() {
                    state_guard.is_open = false;
                }
                let error_msg = format!("Failed to open port {}: {}", port_name_clone, e);
                app_handle_clone.emit("serial_error", error_msg).ok();
                return;
            }
        };
        
        // 发送端口已打开的事件
        let open_msg = format!("Port {} opened at {} bps", port_name_clone, baud_rate);
        app_handle_clone.emit("serial_message", open_msg).ok();
        
        // 读取缓冲区
        let mut buffer = [0; 1024];
        // 累积缓冲区，用于收集完整的数据
        let mut accumulated_data = Vec::new();
        // 上次发送数据的时间
        let mut last_send_time = std::time::Instant::now();
        
        // 持续读取数据
        loop {
            // 检查端口是否仍应打开
            {   
                let state_guard = match state_clone.lock() {
                    Ok(guard) => guard,
                    Err(_) => break,
                };
                if !state_guard.is_open {
                    break;
                }
            }
            
            // 尝试读取数据
            match port.read(&mut buffer) {
                Ok(bytes_read) if bytes_read > 0 => {
                    // 将读取的数据添加到累积缓冲区
                    accumulated_data.extend_from_slice(&buffer[..bytes_read]);
                    
                    // 检查是否应该发送数据：
                    // 1. 缓冲区中有足够的数据（超过10个字节）
                    // 2. 距离上次发送已经超过50ms（即使数据量不大）
                    let current_time = std::time::Instant::now();
                    if accumulated_data.len() >= 10 || current_time.duration_since(last_send_time) > Duration::from_millis(50) {
                        // 尝试将累积的数据转换为字符串
                        if let Ok(message) = String::from_utf8(accumulated_data.clone()) {
                            // 发送数据到前端
                            app_handle_clone.emit("serial_data", message).ok();
                        } else {
                            // 如果不是有效的UTF-8，则发送十六进制表示
                            let hex_data: Vec<String> = accumulated_data
                                .iter()
                                .map(|b| format!("{:02X}", b))
                                .collect();
                            let hex_message = format!("HEX: {}", hex_data.join(" "));
                            app_handle_clone.emit("serial_data", hex_message).ok();
                        }
                        // 清空累积缓冲区并更新上次发送时间
                        accumulated_data.clear();
                        last_send_time = current_time;
                    }
                },
                Ok(_) => {
                    // 没有读取到数据，但检查累积缓冲区是否有数据需要发送
                    let current_time = std::time::Instant::now();
                    if !accumulated_data.is_empty() && current_time.duration_since(last_send_time) > Duration::from_millis(50) {
                        // 发送累积的数据
                        if let Ok(message) = String::from_utf8(accumulated_data.clone()) {
                            app_handle_clone.emit("serial_data", message).ok();
                        } else {
                            let hex_data: Vec<String> = accumulated_data
                                .iter()
                                .map(|b| format!("{:02X}", b))
                                .collect();
                            let hex_message = format!("HEX: {}", hex_data.join(" "));
                            app_handle_clone.emit("serial_data", hex_message).ok();
                        }
                        accumulated_data.clear();
                        last_send_time = current_time;
                    }
                    // 短暂休眠以避免CPU占用过高
                    thread::sleep(Duration::from_millis(10));
                },
                Err(e) => {
                    // 只处理致命错误，忽略临时错误（如超时）
                    match e.kind() {
                        std::io::ErrorKind::TimedOut => {
                            // 超时是正常的，检查累积缓冲区是否有数据需要发送
                            let current_time = std::time::Instant::now();
                            if !accumulated_data.is_empty() && current_time.duration_since(last_send_time) > Duration::from_millis(50) {
                                if let Ok(message) = String::from_utf8(accumulated_data.clone()) {
                                    app_handle_clone.emit("serial_data", message).ok();
                                } else {
                                    let hex_data: Vec<String> = accumulated_data
                                        .iter()
                                        .map(|b| format!("{:02X}", b))
                                        .collect();
                                    let hex_message = format!("HEX: {}", hex_data.join(" "));
                                    app_handle_clone.emit("serial_data", hex_message).ok();
                                }
                                accumulated_data.clear();
                                last_send_time = current_time;
                            }
                            thread::sleep(Duration::from_millis(10));
                        },
                        _ => {
                            // 发送错误信息
                            let error_msg = format!("Error reading from port {}: {}", port_name_clone, e);
                            app_handle_clone.emit("serial_error", error_msg).ok();
                            // 发生致命错误时关闭端口
                            break;
                        }
                    }
                },
            }
        }
        
        // 确保在退出前发送剩余的数据
        if !accumulated_data.is_empty() {
            if let Ok(message) = String::from_utf8(accumulated_data.clone()) {
                app_handle_clone.emit("serial_data", message).ok();
            } else {
                let hex_data: Vec<String> = accumulated_data
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect();
                let hex_message = format!("HEX: {}", hex_data.join(" "));
                app_handle_clone.emit("serial_data", hex_message).ok();
            }
        }
        
        // 更新状态为已关闭
        if let Ok(mut state_guard) = state_clone.lock() {
            state_guard.is_open = false;
        }
        
        // 发送端口已关闭的事件
        let close_msg = format!("Port {} closed", port_name_clone);
        app_handle_clone.emit("serial_message", close_msg).ok();
    });
    
    Ok(format!("Opening port {} at {} bps", port_name, baud_rate))
}

#[tauri::command]
fn close_serial_port(
    state: State<Arc<Mutex<SerialPortState>>>,
) -> Result<String, String> {
    let mut state_guard = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    
    if !state_guard.is_open {
        return Err("No port is currently open".to_string());
    }
    
    let port_name = state_guard.port_name.clone().unwrap_or("unknown".to_string());
    state_guard.is_open = false;
    
    Ok(format!("Closing port {}", port_name))
}

#[tauri::command]
fn get_available_ports() -> Result<Vec<PortInfo>, String> {
    match available_ports() {
        Ok(ports) => {
            let port_infos: Vec<PortInfo> = ports
                .iter()
                .map(|port| PortInfo {
                    name: port.port_name.clone(),
                    port_type: match &port.port_type {
                        serialport::SerialPortType::UsbPort(info) => {
                            if let Some(product) = &info.product {
                                product.clone()
                            } else {
                                format!("{:?}", port.port_type)
                            }
                        },
                        _ => format!("{:?}", port.port_type),
                    },
                })
                .collect();
            Ok(port_infos)
        }
        Err(e) => Err(format!("Failed to list serial ports: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Arc::new(Mutex::new(SerialPortState::default())))
        .invoke_handler(tauri::generate_handler![greet, get_available_ports, open_serial_port, close_serial_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
