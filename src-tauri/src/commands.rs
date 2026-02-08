use crate::core::input_monitor;

#[tauri::command]
pub fn start_input_monitoring() -> Result<String, String> {
    match input_monitor::start_monitoring() {
        Ok(_) => Ok("monitor started".to_string()),
        Err(e) => Err(format!("monitor error: {}", e)),
    }
}

#[tauri::command]
pub fn stop_input_monitoring() -> Result<String, String> {
    input_monitor::stop_monitoring();
    Ok("monitor stopped".to_string())
}
