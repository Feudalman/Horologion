mod app;
mod server;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    app::init_and_run();
}
