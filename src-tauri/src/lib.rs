// Copyright (c) 2026 Simon-Weij
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::Command;
use std::fs;
use serde::{Deserialize, Serialize};
use rfd;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_terminal_command(command: String) -> Result<String, String> {
    match Command::new("sh").arg("-c").arg(&command).output() {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(format!("Failed to execute command: {}", e)),
    }
}

#[derive(Serialize, Deserialize)]
struct Settings {
    backend: String,
    clip_length: u32,
    clip_hotkey: Vec<String>,
    framerate: u32,
    replay_time: u32,
    container: String,
    output: String,
    codec: String,
    quality: u32,
    framerate_mode: String,
    bitrate_mode: String,
}

#[tauri::command]
fn save_settings(backend: String, clip_length: u32, clip_hotkey: Vec<String>,
    framerate: u32, replay_time: u32, container: String, output: String,
    codec: String, quality: u32, framerate_mode: String, bitrate_mode: String) -> Result<(), String> {
    let settings = Settings {
        backend,
        clip_length,
        clip_hotkey,
        framerate,
        replay_time,
        container,
        output,
        codec,
        quality,
        framerate_mode,
        bitrate_mode,
    };
    
    let config_dir = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("flux");
    
    fs::create_dir_all(&config_dir).map_err(|e| format!("Failed to create config directory: {}", e))?;
    
    let settings_path = config_dir.join("settings.json");
    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    fs::write(&settings_path, json).map_err(|e| format!("Failed to write settings file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
fn get_capture_options() -> Result<Vec<String>, String> {
    let output = Command::new("gpu-screen-recorder")
        .arg("--list-capture-options")
        .output()
        .map_err(|e| format!("Failed to run gpu-screen-recorder: {}", e))?;
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut options = vec!["screen".to_string(), "portal".to_string()];
    for line in stdout.lines() {
        if line.contains('|') {
            if let Some(name) = line.split('|').next() {
                options.push(name.trim().to_string());
            }
        }
    }
    Ok(options)
}

#[tauri::command]
fn select_folder() -> Result<String, String> {
    match rfd::FileDialog::new().pick_folder() {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => Err("No folder selected".to_string()),
    }
}

#[tauri::command]
fn load_settings() -> Result<Settings, String> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("flux");
    
    let settings_path = config_dir.join("settings.json");
    
    if !settings_path.exists() {
        return Ok(Settings {
            backend: "gpu-screen-recorder".to_string(),
            clip_length: 30,
            clip_hotkey: vec!["KEY_LEFTALT".to_string(), "KEY_Z".to_string()],
            framerate: 60,
            replay_time: 30,
            container: "mp4".to_string(),
            output: "~/Videos/clip".to_string(),
            codec: "h264".to_string(),
            quality: 20,
            framerate_mode: "vfr".to_string(),
            bitrate_mode: "cqp".to_string(),
        });
    }
    
    let json = fs::read_to_string(&settings_path)
        .map_err(|e| format!("Failed to read settings file: {}", e))?;
    
    serde_json::from_str(&json).map_err(|e| format!("Failed to parse settings: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, run_terminal_command, save_settings, load_settings,
            get_capture_options, select_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
