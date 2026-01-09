// Copyright (c) 2026 Simon-Weij
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::Command;
use std::fs;
use serde::{Deserialize, Serialize};

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
}

#[tauri::command]
fn save_settings(backend: String, clip_length: u32, clip_hotkey: Vec<String>) -> Result<(), String> {
    let settings = Settings {
        backend,
        clip_length,
        clip_hotkey,
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
fn load_settings() -> Result<Settings, String> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("flux");
    
    let settings_path = config_dir.join("settings.json");
    
    if !settings_path.exists() {
        return Ok(Settings {
            backend: "obs".to_string(),
            clip_length: 30,
            clip_hotkey: vec!["KEY_LEFTALT".to_string(), "KEY_Z".to_string()],
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
        .invoke_handler(tauri::generate_handler![greet, run_terminal_command, save_settings, load_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
