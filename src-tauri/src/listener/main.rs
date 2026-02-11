mod window;
use log::{error, info};
use rdev::{listen, Event, EventType};
use std::io::{self, Write};
use window::get_current_window_info;

/// 键鼠监听器
fn main() {
    // 初始化日志
    env_logger::init();

    info!("Input listener process started");

    // 监听输入事件
    if let Err(error) = listen(callback) {
        error!("Listening error: {:?}", error);
        std::process::exit(1);
    }
}

/// 监听回调
fn callback(event: Event) {
    let result = std::panic::catch_unwind(|| {
        // 获取当前活动窗口信息
        let window_info = get_current_window_info();
        let window_json = window_info
            .map(|w| w.to_json())
            .unwrap_or_else(|| "null".to_string());

        match event.event_type {
            EventType::KeyPress(key) => {
                let event_data = format!("KeyPress:{:?}|Window:{}", key, window_json);
                println!("{}", event_data);
                io::stdout().flush().unwrap();
            }
            EventType::KeyRelease(key) => {
                let event_data = format!("KeyRelease:{:?}|Window:{}", key, window_json);
                println!("{}", event_data);
                io::stdout().flush().unwrap();
            }
            EventType::ButtonPress(button) => {
                let event_data = format!("ButtonPress:{:?}|Window:{}", button, window_json);
                println!("{}", event_data);
                io::stdout().flush().unwrap();
            }
            EventType::ButtonRelease(button) => {
                let event_data = format!("ButtonRelease:{:?}|Window:{}", button, window_json);
                println!("{}", event_data);
                io::stdout().flush().unwrap();
            }
            EventType::MouseMove { x: _, y: _ } => {
                // 鼠标移动事件太频繁，可以选择性记录
                // let event_data = format!("MouseMove:{}:{}|Window:{}", x, y, window_json);
                // println!("{}", event_data);
            }
            EventType::Wheel { delta_x, delta_y } => {
                let event_data = format!("Wheel:{}:{}|Window:{}", delta_x, delta_y, window_json);
                println!("{}", event_data);
                io::stdout().flush().unwrap();
            }
        }
    });

    if let Err(e) = result {
        eprintln!("Callback panicked: {:?}", e);
    }
}
