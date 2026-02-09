use rdev::{listen, Event, EventType};
use log::{info, error};
use std::io::{self, Write};

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

fn callback(event: Event) {
    let result = std::panic::catch_unwind(|| {
        match event.event_type {
            EventType::KeyPress(key) => {
                let event_data = format!("KeyPress:{:?}", key);
                println!("{}", event_data);
                io::stdout().flush().unwrap();
            }
            EventType::KeyRelease(key) => {
                let event_data = format!("KeyRelease:{:?}", key);
                println!("{}", event_data);
                io::stdout().flush().unwrap();
            }
            EventType::ButtonPress(button) => {
                let event_data = format!("ButtonPress:{:?}", button);
                println!("{}", event_data);
                io::stdout().flush().unwrap();
            }
            EventType::ButtonRelease(button) => {
                let event_data = format!("ButtonRelease:{:?}", button);
                println!("{}", event_data);
                io::stdout().flush().unwrap();
            }
            EventType::MouseMove { x, y } => {
                // 鼠标移动事件太频繁，可以选择性记录
                // let event_data = format!("MouseMove:{}:{}", x, y);
                // println!("{}", event_data);
            }
            EventType::Wheel { delta_x, delta_y } => {
                let event_data = format!("Wheel:{}:{}", delta_x, delta_y);
                println!("{}", event_data);
                io::stdout().flush().unwrap();
            }
        }
    });

    if let Err(e) = result {
        eprintln!("Callback panicked: {:?}", e);
    }
}
