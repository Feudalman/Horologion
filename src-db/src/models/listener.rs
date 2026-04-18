//! `src-listener` 采集数据对应的模型。

use serde::{Deserialize, Serialize};

/// `src-listener` 产生的输入事件类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputEventKind {
    KeyPress,
    KeyRelease,
    ButtonPress,
    ButtonRelease,
    Wheel,
}

impl InputEventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeyPress => "key_press",
            Self::KeyRelease => "key_release",
            Self::ButtonPress => "button_press",
            Self::ButtonRelease => "button_release",
            Self::Wheel => "wheel",
        }
    }
}

/// 输入事件发生时采集到的活动窗口快照。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservedWindow {
    pub app_name: String,
    pub process_path: Option<String>,
    pub process_id: Option<u64>,
    pub title: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

/// 准备写入数据库的输入事件载荷。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputEvent {
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub kind: InputEventKind,
    pub value: Option<String>,
    pub delta_x: Option<f64>,
    pub delta_y: Option<f64>,
    pub window: Option<ObservedWindow>,
    pub raw_event: Option<String>,
    pub raw_window: Option<String>,
}
