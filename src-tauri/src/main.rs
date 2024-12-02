// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{Emitter, Manager, State};
use tokio::sync::mpsc::{self, Sender};
use tokio::task;

#[derive(Serialize, Deserialize, Debug)]
struct ServoCommand {
    id: u8,
    angle: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct DigitalOutputCommand {
    pin: u8,
    state: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct StatusUpdate {
    digital_inputs: Vec<DigitalInputStatus>,
    status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DigitalInputStatus {
    pin: u8,
    state: bool,
}

struct AppState {
    port: Arc<Mutex<Option<Box<dyn SerialPort + Send>>>>,
    sender: Mutex<Option<Sender<String>>>,
}

#[tokio::main]
async fn main() {
    let app_state = AppState {
        port: Arc::new(Mutex::new(None)),
        sender: Mutex::new(None),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            list_serial_ports,
            connect_port,
            send_servo_command,
            send_digital_output
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let state = app_handle.state::<AppState>();
            let port_clone = Arc::clone(&state.port);
            let (tx, mut rx) = mpsc::channel::<String>(100);
            {
                let mut sender_lock = state.sender.lock().unwrap();
                *sender_lock = Some(tx.clone());
            }

            // Task to handle incoming messages and emit to frontend
            task::spawn(async move {
                while let Some(message) = rx.recv().await {
                    if let Err(e) = app_handle.emit("status_update", message) {
                        eprintln!("Failed to emit status_update: {:?}", e);
                    }
                }
            });

            // Task to read from serial port
            task::spawn(async move {
                loop {
                    let data = {
                        let mut port = port_clone.lock().unwrap();
                        if let Some(ref mut port) = *port {
                            let mut buffer: Vec<u8> = vec![0; 1024];
                            match port.read(buffer.as_mut_slice()) {
                                Ok(bytes_read) if bytes_read > 0 => {
                                    Some(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    };

                    if let Some(data) = data {
                        if let Err(e) = tx.send(data).await {
                            eprintln!("Failed to send data through channel: {:?}", e);
                        }
                    }

                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn list_serial_ports() -> Result<Vec<String>, String> {
    serialport::available_ports()
        .map_err(|e| e.to_string())
        .map(|ports| {
            ports
                .into_iter()
                .filter_map(|port| Some(port.port_name))
                .collect::<Vec<String>>()
        })
}

#[tauri::command]
fn connect_port(state: State<AppState>, port: String) -> Result<String, String> {
    let port_result = serialport::new(port, 9600)
        .timeout(Duration::from_millis(1000))
        .open();

    match port_result {
        Ok(serial_port) => {
            let mut locked_port = state.port.lock().unwrap();
            *locked_port = Some(serial_port);
            Ok("Connected".into())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn send_servo_command(state: State<AppState>, command: ServoCommand) -> Result<(), String> {
    let mut locked_port = state.port.lock().unwrap();
    if let Some(ref mut port) = *locked_port {
        let cmd = format!("S{}:{}\n", command.id, command.angle);
        port.write(cmd.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Port not connected".into())
    }
}

#[tauri::command]
fn send_digital_output(
    state: State<AppState>,
    command: DigitalOutputCommand,
) -> Result<(), String> {
    let mut locked_port = state.port.lock().unwrap();
    if let Some(ref mut port) = *locked_port {
        let state_str = if command.state { "HIGH" } else { "LOW" };
        let cmd = format!("D{}:{}\n", command.pin, state_str);
        port.write(cmd.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Port not connected".into())
    }
}
