//! Tauri command 路由绑定。
//!
//! 所有暴露给前端的 command 都集中在这里注册，便于后续检查前端可调用的接口清单。

use tauri::{ipc::Invoke, Runtime};

use crate::server::{events, settings, windows};

/// 统一绑定 server 模块下暴露给前端的 Tauri commands。
pub fn handler<R: Runtime>() -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        settings::get_app_status,
        settings::get_app_settings,
        settings::get_database_file_size,
        settings::start_listener,
        settings::stop_listener,
        events::get_activity_summary,
        events::list_input_events,
        events::get_input_event,
        windows::list_observed_windows,
        windows::get_observed_window
    ]
}
