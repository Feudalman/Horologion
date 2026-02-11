use active_win_pos_rs::get_active_window;
use log::{error, warn};
use serde_json::json;

/// 窗口信息结构体
#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub title: String,
    pub app_name: String,
    pub process_path: String,
    pub process_id: u32,
    pub position: (i32, i32),
    pub size: (u32, u32),
}

impl WindowInfo {
    /// 转换为 JSON 字符串
    pub fn to_json(&self) -> String {
        json!({
            "title": self.title,
            "app_name": self.app_name,
            "process_path": self.process_path,
            "process_id": self.process_id,
            "position": {
                "x": self.position.0,
                "y": self.position.1
            },
            "size": {
                "width": self.size.0,
                "height": self.size.1
            }
        }).to_string()
    }
}

/// 获取当前活动窗口信息
pub fn get_current_window_info() -> Option<WindowInfo> {
    match get_active_window() {
        Ok(active_window) => {
            Some(WindowInfo {
                title: active_window.title,
                app_name: active_window.app_name,
                process_path: active_window.process_path,
                process_id: active_window.process_id,
                position: (active_window.position.x, active_window.position.y),
                size: (active_window.position.width, active_window.position.height),
            })
        }
        Err(e) => {
            warn!("Failed to get active window: {}", e);
            None
        }
    }
}
