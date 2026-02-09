use crate::commands;
use crate::core::input_monitor;
use log::{error, info};

pub fn init_and_run() {
    // 检查是否手动设置了日志级别，如果没有，则设置为 info
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    // 初始化日志
    env_logger::init();
    
    // 自动启动键鼠事件监听
    if let Err(e) = input_monitor::start_monitoring() {
        error!("auto start input monitoring error: {}", e);
    } else {
        info!("auto start input monitoring success");
    }
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::start_input_monitoring,
            commands::stop_input_monitoring
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
