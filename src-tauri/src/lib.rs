mod core;

use core::input_monitor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start_input_monitoring() -> Result<String, String> {
    match input_monitor::start_monitoring() {
        Ok(_) => Ok("键鼠事件监听已启动".to_string()),
        Err(e) => Err(format!("启动监听失败: {}", e)),
    }
}

#[tauri::command]
fn stop_input_monitoring() -> Result<String, String> {
    input_monitor::stop_monitoring();
    Ok("键鼠事件监听已停止".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    env_logger::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start_input_monitoring, stop_input_monitoring])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
