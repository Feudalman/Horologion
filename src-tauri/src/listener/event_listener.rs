use chrono::{DateTime, Local};
use log::{error, info};
use rdev::{listen, Event, EventType};
use std::io::{self, Write};

use crate::window::get_current_window_info;

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

    /// 监听回调
    fn callback(event: Event) {
        let result = std::panic::catch_unwind(|| {
            // 获取当前时间
            let timestamp: DateTime<Local> = Local::now();
            let time_str = timestamp.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
            
            // 获取当前活动窗口信息
            let window_info = get_current_window_info();
            let window_json = window_info
                .map(|w| w.to_json())
                .unwrap_or_else(|| "null".to_string());

            match event.event_type {
                EventType::KeyPress(key) => {
                    let event_data = format!("{}|KeyPress:{:?}|Window:{}", time_str, key, window_json);
                    println!("{}", event_data);
                    io::stdout().flush().unwrap();
                }
                EventType::KeyRelease(key) => {
                    let event_data = format!("{}|KeyRelease:{:?}|Window:{}", time_str, key, window_json);
                    println!("{}", event_data);
                    io::stdout().flush().unwrap();
                }
                EventType::ButtonPress(button) => {
                    let event_data = format!("{}|ButtonPress:{:?}|Window:{}", time_str, button, window_json);
                    println!("{}", event_data);
                    io::stdout().flush().unwrap();
                }
                EventType::ButtonRelease(button) => {
                    let event_data = format!("{}|ButtonRelease:{:?}|Window:{}", time_str, button, window_json);
                    println!("{}", event_data);
                    io::stdout().flush().unwrap();
                }
                EventType::MouseMove { x: _, y: _ } => {
                    // 鼠标移动事件太频繁，可以选择性记录
                    // let event_data = format!("{}|MouseMove:{}:{}|Window:{}", time_str, x, y, window_json);
                    // println!("{}", event_data);
                }
                EventType::Wheel { delta_x, delta_y } => {
                    let event_data = format!("{}|Wheel:{}:{}|Window:{}", time_str, delta_x, delta_y, window_json);
                    println!("{}", event_data);
                    io::stdout().flush().unwrap();
                }
            }
        });

        if let Err(e) = result {
            eprintln!("Callback panicked: {:?}", e);
        }
    }
}
