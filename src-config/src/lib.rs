//! Shared configuration constants for Horologion Rust crates.

pub mod app {
    pub const NAME: &str = "horologion";
    pub const DISPLAY_NAME: &str = "Horologion";
}

pub mod database {
    pub const FILE_NAME: &str = "horologion.db";
    pub const MEMORY_CONNECTION: &str = ":memory:";
}

pub mod env {
    pub const DATABASE_PATH: &str = "DATABASE_PATH";
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const LISTENER_TRANSPORT: &str = "HOROLOGION_LISTENER_TRANSPORT";
    pub const LOG_LEVEL: &str = "LOG_LEVEL";
    pub const RUN_MODE: &str = "RUN_MODE";
    pub const RUST_LOG: &str = "RUST_LOG";
}

pub mod logging {
    pub const APP_BASENAME: &str = "app";
    pub const DIRECTORY_NAME: &str = "logs";
    pub const LISTENER_BASENAME: &str = "listener";
    pub const RETENTION_DAYS: usize = 30;
    pub const ROTATE_SIZE_BYTES: u64 = 10 * 1024 * 1024;
}

pub mod listener {
    pub const BINARY_NAME: &str = "listener";
}

pub mod pagination {
    pub const DEFAULT_PAGE: i64 = 1;
    pub const DEFAULT_PAGE_SIZE: i64 = 100;
    pub const MAX_PAGE_SIZE: i64 = 1_000;
}

pub mod overview {
    pub const TOP_APPS_LIMIT: i64 = 10;
}

pub mod paths {
    pub const CONFIG_FILE_NAME: &str = "config.toml";
    pub const PLAYGROUND_DIR: &str = "playground";
    pub const DATABASE_DIR: &str = "db";
    pub const PERMISSIONS_DIR: &str = "permissions";
}

pub mod protocol {
    pub const LISTENER_STDIO_EVENT_PREFIX: &str = "__HOROLOGION_INPUT_EVENT__";
    pub const LISTENER_TRANSPORT_STDIO: &str = "stdio";
}

pub mod tauri {
    pub const MAIN_WINDOW_LABEL: &str = "main";
    pub const WINDOW_STATE_FILE: &str = "window-state.json";

    pub mod tray {
        pub const ID: &str = "horologion-tray";
        pub const SHOW_ID: &str = "tray-show-window";
        pub const START_LISTENER_ID: &str = "tray-start-listener";
        pub const STOP_LISTENER_ID: &str = "tray-stop-listener";
        pub const QUIT_ID: &str = "tray-quit";
    }
}
