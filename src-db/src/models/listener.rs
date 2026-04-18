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
    /// 活动窗口所属应用名称。
    pub app_name: String,
    /// 应用进程的可执行文件路径。
    pub process_path: Option<String>,
    /// 操作系统分配的进程 ID。
    pub process_id: Option<u64>,
    /// 活动窗口标题。
    pub title: String,
    /// 窗口左上角横坐标。
    pub x: Option<f64>,
    /// 窗口左上角纵坐标。
    pub y: Option<f64>,
    /// 窗口宽度。
    pub width: Option<f64>,
    /// 窗口高度。
    pub height: Option<f64>,
}

/// 准备写入数据库的输入事件载荷。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputEvent {
    /// 事件实际发生时间，统一使用 UTC 保存。
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    /// 输入事件类型。
    pub kind: InputEventKind,
    /// 事件值：按键名、鼠标按钮名或滚轮方向描述。
    pub value: String,
    /// 滚轮横向滚动距离。
    pub delta_x: Option<f64>,
    /// 滚轮纵向滚动距离。
    pub delta_y: Option<f64>,
    /// 事件发生时的活动窗口快照。
    pub window: Option<ObservedWindow>,
    /// 原始事件 JSON，便于调试和后续补字段。
    pub raw_event: Option<String>,
    /// 原始窗口 JSON，便于调试和后续补字段。
    pub raw_window: Option<String>,
}
