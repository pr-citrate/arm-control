// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serialport::{SerialPort, SerialPortType};
use std::sync::Mutex;
use std::time::Duration;
use tauri::State;

// 상태 구조체
#[derive(Default)]
pub struct AppState {
    port: Mutex<Option<Box<dyn SerialPort>>>,
}

// 서보 명령 구조체
#[derive(Serialize, Deserialize)]
struct ServoCommand {
    angles: Vec<u8>,            // 6개 서보 각도
    digital_outputs: Vec<bool>, // 3개 디지털 출력
}

// 아두이노 상태 구조체
#[derive(Serialize, Deserialize)]
struct ArduinoStatus {
    servo_angles: Vec<u8>,      // 6개 서보 각도
    digital_outputs: Vec<bool>, // 3개 디지털 출력
    digital_inputs: Vec<bool>,  // 3개 디지털 입력
}

#[tauri::command]
fn list_serial_ports() -> Result<Vec<String>, String> {
    let ports = serialport::available_ports().map_err(|e| e.to_string())?;
    Ok(ports
        .into_iter()
        .filter_map(|p| match p.port_type {
            SerialPortType::UsbPort(_) => Some(p.port_name),
            _ => None,
        })
        .collect())
}

#[tauri::command]
fn connect_port(state: State<AppState>, port: &str) -> Result<String, String> {
    match serialport::new(port, 9600)
        .timeout(Duration::from_millis(1000))
        .open()
    {
        Ok(mut serial_port) => {
            // 테스트 메시지 전송
            let test_cmd = "S90,90,90,90,90,90,0,0,0E\n";
            serial_port
                .write_all(test_cmd.as_bytes())
                .map_err(|e| format!("Failed to write test message: {}", e))?;
            serial_port
                .flush()
                .map_err(|e| format!("Failed to flush: {}", e))?;

            let mut locked_port = state.port.lock().unwrap();
            *locked_port = Some(serial_port);
            Ok("Connected".into())
        }
        Err(e) => Err(format!("Failed to open port {}: {}", port, e)),
    }
}

#[tauri::command]
fn send_command(state: State<AppState>, command: ServoCommand) -> Result<(), String> {
    let mut locked_port = state.port.lock().unwrap();
    if let Some(ref mut port) = *locked_port {
        let mut cmd = String::new();

        cmd.push('S');

        // 서보 각도 추가
        for angle in &command.angles {
            cmd.push_str(&angle.to_string());
            cmd.push(',');
        }

        // 디지털 출력 추가
        for output in &command.digital_outputs {
            cmd.push_str(if *output { "1" } else { "0" });
            cmd.push(',');
        }

        // 마지막 쉼표 제거하고 종료 비트 추가
        cmd.pop(); // 마지막 쉼표 제거
        cmd.push('E');
        cmd.push('\n');

        // 명령 전송
        port.write_all(cmd.as_bytes())
            .map_err(|e| format!("Failed to write command: {}", e))?;
        port.flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;

        // 콘솔에 전송된 명령 출력
        println!("Sent command: {}", cmd);

        Ok(())
    } else {
        Err("Port not connected".into())
    }
}

#[tauri::command]
fn read_status(state: State<AppState>) -> Result<ArduinoStatus, String> {
    let mut locked_port = state.port.lock().unwrap();
    if let Some(ref mut port) = *locked_port {
        let mut buffer = [0u8; 1024];

        // 버퍼 비우기
        while port.bytes_to_read().map_err(|e| e.to_string())? > 0 {
            let _ = port.read(&mut buffer);
        }

        // 상태 요청 명령 전송 (모든 서보 90도, 모든 출력 LOW)
        let request = "S90,90,90,90,90,90,0,0,0E\n";
        port.write_all(request.as_bytes())
            .map_err(|e| format!("Failed to write status request: {}", e))?;
        port.flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;

        // 응답 대기
        std::thread::sleep(Duration::from_millis(50));

        // 응답 읽기
        let n = port
            .read(&mut buffer)
            .map_err(|e| format!("Failed to read from port: {}", e))?;

        let response = String::from_utf8_lossy(&buffer[..n]);

        // 유효한 응답 찾기
        let response = response
            .lines()
            .find(|line| line.starts_with('S') && line.ends_with('E'))
            .ok_or_else(|| "No valid status message found".to_string())?;

        // 응답 파싱
        if response.starts_with('S') && response.ends_with('E') {
            let data = &response[1..response.len() - 1];
            let values: Vec<&str> = data.split(',').collect();

            if values.len() >= 12 {
                let servo_angles: Vec<u8> =
                    values[..6].iter().filter_map(|s| s.parse().ok()).collect();

                let digital_outputs: Vec<bool> = values[6..9].iter().map(|s| s == &"1").collect();

                let digital_inputs: Vec<bool> = values[9..12].iter().map(|s| s == &"1").collect();
                println!("Servo Angles: {:?}", servo_angles);
                println!("Digital Outputs: {:?}", digital_outputs);
                println!("Digital Inputs: {:?}", digital_inputs);
                Ok(ArduinoStatus {
                    servo_angles,
                    digital_outputs,
                    digital_inputs,
                })
            } else {
                Err("Invalid status format".into())
            }
        } else {
            Err("Invalid status message".into())
        }
    } else {
        Err("Port not connected".into())
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            list_serial_ports,
            connect_port,
            send_command,
            read_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
