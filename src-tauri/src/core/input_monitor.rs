use rdev::{listen, Event, EventType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use log::{info, error};

static MONITORING: AtomicBool = AtomicBool::new(false);

pub fn start_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    // 重复启动，不做处理
    if MONITORING.load(Ordering::Relaxed) {
        return Err("monitor is already running".into());
    }
    
    MONITORING.store(true, Ordering::Relaxed);
    info!("monitoring...");
    
    thread::spawn(|| {
        info!("monitoring thread started");
        match listen(callback) {
            Ok(_) => {
                info!("exiting monitoring thread");
            }
            Err(error) => {
                error!("monitoring error: {:?}", error);
                MONITORING.store(false, Ordering::Relaxed);
            }
        }
        info!("monitoring thread exited");
    });
    
    Ok(())
}

pub fn stop_monitoring() {
    MONITORING.store(false, Ordering::Relaxed);
    info!("monitor stopped");
}

fn callback(event: Event) {
    if !MONITORING.load(Ordering::Relaxed) {
        return;
    }

    let result = std::panic::catch_unwind(|| {
        if !MONITORING.load(Ordering::Relaxed) {
            return;
        }

        match event.event_type {
            EventType::KeyPress(key) => {
                // 避免使用 event.name，因为在 macOS 上可能触发主线程断言
                info!("keyboard pressed: {:?}", key);
            }
            EventType::KeyRelease(key) => {
                info!("keyboard released: {:?}", key);
            }
            EventType::ButtonPress(button) => {
                info!("mouse pressed: {:?}", button);
            }
            EventType::ButtonRelease(button) => {
                info!("mouse released: {:?}", button);
            }
            EventType::MouseMove { x, y } => {
                // 鼠标移动事件太频繁，可以选择性记录
                // info!("鼠标移动到: ({}, {})", x, y);
            }
            EventType::Wheel { delta_x, delta_y } => {
                info!("mouse scroll: delta_x={}, delta_y={}", delta_x, delta_y);
            }
        }
    });

    if let Err(e) = result {
        eprintln!("Callback panicked: {:?}", e);
    }
}