use active_win_pos_rs::get_active_window;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub title: String,
    pub app_name: String,
    pub process_path: String,
    pub process_id: u64,
    pub position: (f64, f64),
    pub size: (f64, f64),
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
        })
        .to_string()
    }
}

/// 获取当前活动窗口信息
pub fn get_current_window_info() -> Option<WindowInfo> {
    match get_active_window() {
        Ok(active_window) => {
            let title = non_empty_string(active_window.title)
                .or_else(|| get_focused_window_title(active_window.process_id))
                .unwrap_or_default();

            Some(WindowInfo {
                title,
                app_name: active_window.app_name,
                process_path: active_window.process_path.to_string_lossy().to_string(),
                process_id: active_window.process_id,
                position: (active_window.position.x, active_window.position.y),
                size: (active_window.position.width, active_window.position.height),
            })
        }
        Err(_) => None,
    }
}

#[cfg(target_os = "macos")]
fn get_focused_window_title(process_id: u64) -> Option<String> {
    macos_accessibility_focused_window_title(process_id)
        .inspect(|title| log::debug!("Read window title from macOS Accessibility: {}", title))
}

#[cfg(not(target_os = "macos"))]
fn get_focused_window_title(_process_id: u64) -> Option<String> {
    None
}

fn non_empty_string(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

#[cfg(target_os = "macos")]
fn macos_accessibility_focused_window_title(process_id: u64) -> Option<String> {
    use core_foundation::base::{CFRelease, TCFType};
    use core_foundation::string::CFString;
    use core_foundation_sys::base::{CFTypeRef, OSStatus};
    use core_foundation_sys::string::CFStringRef;
    use std::ffi::c_void;

    type AXUIElementRef = *const c_void;

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn AXUIElementCreateApplication(pid: i32) -> AXUIElementRef;
        fn AXUIElementCopyAttributeValue(
            element: AXUIElementRef,
            attribute: CFStringRef,
            value: *mut CFTypeRef,
        ) -> OSStatus;
    }

    const AX_ERROR_SUCCESS: OSStatus = 0;

    unsafe {
        let app = AXUIElementCreateApplication(process_id as i32);
        if app.is_null() {
            return None;
        }

        let focused_window_attr = CFString::new("AXFocusedWindow");
        let mut focused_window: CFTypeRef = std::ptr::null();
        let focused_window_status = AXUIElementCopyAttributeValue(
            app,
            focused_window_attr.as_concrete_TypeRef(),
            &mut focused_window,
        );
        CFRelease(app as CFTypeRef);

        if focused_window_status != AX_ERROR_SUCCESS || focused_window.is_null() {
            log::debug!(
                "Failed to read AXFocusedWindow for pid {}: {}",
                process_id,
                focused_window_status
            );
            return None;
        }

        let title_attr = CFString::new("AXTitle");
        let mut title_value: CFTypeRef = std::ptr::null();
        let title_status = AXUIElementCopyAttributeValue(
            focused_window as AXUIElementRef,
            title_attr.as_concrete_TypeRef(),
            &mut title_value,
        );
        CFRelease(focused_window);

        if title_status != AX_ERROR_SUCCESS || title_value.is_null() {
            log::debug!(
                "Failed to read AXTitle for pid {}: {}",
                process_id,
                title_status
            );
            return None;
        }

        let title = CFString::wrap_under_create_rule(title_value as CFStringRef).to_string();
        non_empty_string(title)
    }
}
