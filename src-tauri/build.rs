use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // 告诉 Cargo 当 src-listener 目录下的文件发生变化时重新构建
    println!("cargo:rerun-if-changed=../src-listener");

    // 编译 listener
    build_listener();

    tauri_build::build()
}

fn build_listener() {
    let listener_dir = Path::new("../src-listener");
    let bin_dir = Path::new("bin");

    // 确保 bin 目录存在
    if !bin_dir.exists() {
        std::fs::create_dir_all(bin_dir).expect("Failed to create bin directory");
    }

    // 获取目标平台信息
    let target_triple = env::var("TARGET").unwrap_or_else(|_| {
        // 如果没有设置 TARGET，使用当前平台
        if cfg!(target_os = "windows") {
            if cfg!(target_arch = "x86_64") {
                "x86_64-pc-windows-msvc".to_string()
            } else {
                "aarch64-pc-windows-msvc".to_string()
            }
        } else if cfg!(target_os = "macos") {
            if cfg!(target_arch = "x86_64") {
                "x86_64-apple-darwin".to_string()
            } else {
                "aarch64-apple-darwin".to_string()
            }
        } else {
            if cfg!(target_arch = "x86_64") {
                "x86_64-unknown-linux-gnu".to_string()
            } else {
                "aarch64-unknown-linux-gnu".to_string()
            }
        }
    });

    // 构建 listener
    let mut cmd = Command::new("cargo");
    cmd.current_dir(listener_dir)
        .args(&["build", "--release", "--target", &target_triple]);

    let output = cmd
        .output()
        .expect("Failed to execute cargo build for listener");

    if !output.status.success() {
        panic!(
            "Failed to build listener:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // 确定可执行文件名和目标文件名
    let exe_name = if cfg!(target_os = "windows") {
        "listener.exe"
    } else {
        "listener"
    };

    // 根据 Tauri sidecar 命名规范生成目标文件名
    let target_exe_name = format!(
        "listener-{}{}",
        target_triple,
        if cfg!(target_os = "windows") {
            ".exe"
        } else {
            ""
        }
    );

    // 源文件路径
    let source_path = listener_dir
        .join("target")
        .join(&target_triple)
        .join("release")
        .join(exe_name);

    // 目标文件路径
    let target_path = bin_dir.join(&target_exe_name);

    // 复制可执行文件
    if let Err(e) = std::fs::copy(&source_path, &target_path) {
        panic!(
            "Failed to copy listener executable from {:?} to {:?}: {}",
            source_path, target_path, e
        );
    }

    println!(
        "Successfully built and copied listener to {:?}",
        target_path
    );
}
