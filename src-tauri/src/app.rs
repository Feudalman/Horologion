use tauri::window;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

/// 初始化并运行应用
pub fn init_and_run() {
    // 检查是否手动设置了日志级别，如果没有，则设置为 info
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    // 初始化日志
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let sidecar_command = app.shell().sidecar("listener").unwrap();
            let (mut rx, mut child) = sidecar_command.spawn().expect("Failed to spawn sidecar");

            tauri::async_runtime::spawn(async move {
                // 读取诸如 stdout 之类的事件
                while let Some(event) = rx.recv().await {
                    if let CommandEvent::Stdout(line) = event {
                        window
                            .emit("message", Some(format!("'{}'", line)))
                            .expect("failed to emit event");
                        // 写入 stdin
                        child.write("message from Rust\n".as_bytes()).unwrap();
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
