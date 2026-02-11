use crate::window::{get_current_window_info, WindowInfo};
use chrono::{DateTime, Local};
use log::{error, info};
use rdev::{listen, Event, EventType};
use std::io::{self, Write};

/// 事件监听器
pub struct EventListener;

impl EventListener {
    /// 创建新的事件监听器
    pub fn new() -> Self {
        Self
    }

    /// 启动监听
    pub fn start(&self) -> Result<(), String> {
        info!("Input listener process started");

        if let Err(error) = listen(Self::callback) {
            let error_msg = format!("Listening error: {:?}", error);
            error!("{}", error_msg);
            return Err(error_msg);
        }

        Ok(())
    }

    /// 获取当前时间戳和窗口信息
    fn get_event_context() -> (String, String, Option<WindowInfo>) {
        // 获取当前时间
        let timestamp: DateTime<Local> = Local::now();
        let time_str = timestamp.format("%Y-%m-%d %H:%M:%S%.3f").to_string();

        // 获取当前活动窗口信息
        let window_info = get_current_window_info();
        let window_json = window_info
            .clone()
            .map(|w| w.to_json())
            .unwrap_or_else(|| "{}".to_string());

        (time_str, window_json, window_info)
    }

    /// 输出事件数据
    fn output_event(event_type: &str, event_detail: &str, time_str: &str, window_json: &str) {
        let event_data = format!(
            "{}|{}:{}|Window:{}",
            time_str, event_type, event_detail, window_json
        );
        println!("{}", event_data);
        io::stdout().flush().unwrap();
    }

    /// 监听回调
    fn callback(event: Event) {
        let result = std::panic::catch_unwind(|| {
            // 鼠标移动事件太频繁，直接跳过处理
            if matches!(event.event_type, EventType::MouseMove { .. }) {
                return;
            }

            let (time_str, window_json, _) = Self::get_event_context();

            match event.event_type {
                EventType::KeyPress(key) => {
                    Self::output_event("KeyPress", &format!("{:?}", key), &time_str, &window_json);
                }
                EventType::KeyRelease(key) => {
                    Self::output_event("KeyRelease", &format!("{:?}", key), &time_str, &window_json);
                }
                EventType::ButtonPress(button) => {
                    Self::output_event("ButtonPress", &format!("{:?}", button), &time_str, &window_json);
                }
                EventType::ButtonRelease(button) => {
                    Self::output_event("ButtonRelease", &format!("{:?}", button), &time_str, &window_json);
                }
                EventType::Wheel { delta_x, delta_y } => {
                    Self::output_event("Wheel", &format!("delta_x:{}, delta_y:{}", delta_x, delta_y), &time_str, &window_json);
                }
                _ => {}
            }
        });

        if result.is_err() {
            error!("Error in event callback");
        }
    }
}
