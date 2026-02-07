use rdev::{listen, Event, EventType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use log::{info, error, warn};

static MONITORING: AtomicBool = AtomicBool::new(false);

pub fn start_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    if MONITORING.load(Ordering::Relaxed) {
        return Err("监听已经在运行中".into());
    }
    
    MONITORING.store(true, Ordering::Relaxed);
    info!("开始启动键鼠事件监听...");
    
    thread::spawn(|| {
        info!("监听线程已启动");
        match listen(callback) {
            Ok(_) => {
                info!("监听正常结束");
            }
            Err(error) => {
                error!("监听错误: {:?}", error);
                MONITORING.store(false, Ordering::Relaxed);
            }
        }
        info!("监听线程已退出");
    });
    
    info!("键鼠事件监听已启动");
    Ok(())
}

pub fn stop_monitoring() {
    MONITORING.store(false, Ordering::Relaxed);
    info!("键鼠事件监听已停止");
}

fn callback(event: Event) -> Option<Event> {
    if !MONITORING.load(Ordering::Relaxed) {
        return Some(event);
    }
    
    match event.event_type {
        EventType::KeyPress(key) => {
            info!("键盘按下: {:?}, event name: {:?}", key, event.name);
        }
        EventType::KeyRelease(key) => {
            info!("键盘释放: {:?}, event name: {:?}", key, event.name);
        }
        EventType::ButtonPress(button) => {
            info!("鼠标按下: {:?}, event name: {:?}", button, event.name);
        }
        EventType::ButtonRelease(button) => {
            info!("鼠标释放: {:?}, event name: {:?}", button, event.name);
        }
        EventType::MouseMove { x, y } => {
            // 鼠标移动事件太频繁，可以选择性记录
            // info!("鼠标移动到: ({}, {})", x, y);
        }
        EventType::Wheel { delta_x, delta_y } => {
            info!("鼠标滚轮: delta_x={}, delta_y={}", 
                  delta_x, delta_y);
        }
    }
    
    // 返回 Some(event) 允许事件继续传播到系统
    Some(event)
}
