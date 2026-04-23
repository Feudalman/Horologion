<p align="center">
  <img src="logo.png" alt="Horologion logo" width="120" style="border-radius: 24px;" />
</p>

# Horologion

[中文说明](README-ZH.md)

Horologion is a local activity timeline tool that is still in its early stage. The current version has implemented the basic collection, storage, and viewing pipeline, but it is not yet a mature production-grade application. Error handling, logging, database rotation, database cleanup, and backup workflows are still incomplete. Please use it carefully.

Before running it for the first time, please pay special attention to the following:

- Horologion continuously records local keyboard events, mouse button events, wheel events, and the active window context at the time of each event. Window context may include the application name, window title, process path, and process ID.
- The current database is stored as a local DuckDB file. The data will keep growing as the application runs. There is no automatic rotation, compression, quick backup, quick cleanup, or retention policy yet.
- If the database file becomes too large, you need to manually delete, move, or back it up. Deleting the database file will remove the collected data.
- Configuration-file-driven behavior is still experimental. The interface and format may change in later versions. At this stage, the project mainly supports the default log output level and the default local database path. More configuration options will be improved in future releases.
- This project collects local interaction data and the open-source version does not sync data to the cloud by default. Do not accidentally commit or share database files that contain personal activity records.

## Overview

Horologion aims to become a cross-platform local activity recording and observation tool. It listens for basic input events in the background, writes those events together with the active window context to a local database, and displays them in a desktop app as a searchable, paginated, and aggregatable timeline.

You can think of it as an activity foundation for personal research and local automation experiments. It does not try to judge what you are doing. It first focuses on reliably preserving what input event happened, when it happened, and which foreground window was active at that moment. Future versions can build on this foundation to support statistics, automatic archiving, activity replay, time accounting, or more detailed personal workflow tools.

The name Horologion comes from the idea of timekeeping and recording moments. In Chinese, it can also be understood as "日晷" (sundial): a trace left by time.

## Current Features

- Desktop app shell built with Tauri 2.
- Frontend built with React, Vite, TypeScript, and Tailwind CSS.
- Local event listener through an independent listener sidecar for keyboard events, mouse button events, and wheel events.
- Active window snapshots that record the foreground application, window title, process path, process ID, window position, and window size at the time of each event.
- Local database storage with DuckDB for input events and observed windows.

Mouse movement events are not recorded at this stage because they are extremely high frequency and can quickly increase the database size.

## Current Limitations

The goal of this stage is to make the main pipeline work end to end. The following areas are still incomplete:

- Error handling is still rough, and some failure cases are only printed to the terminal or logs.
- Logging is still based on a basic `env_logger` setup. The default log level is `info`, and there is no in-app log viewer, log file management, or structured logging yet.
- The database does not yet support automatic rotation, archiving, cleanup, compression, or quick backup.
- The database file size is only shown in the settings page. The app does not actively warn about or limit database growth.
- Configuration file support is still experimental. Please do not treat it as a stable interface yet.
- Packaging, distribution, permission guidance, and cross-platform behavior still need more work.
- Collection accuracy depends on operating system permissions and third-party libraries, so behavior may vary across platforms.

## Privacy And Data

Horologion stores its core data in a local database. The current tables mainly include:

- `input_events`: input event timestamp, event type, event value, wheel delta, raw event JSON, collector metadata, and the associated window ID.
- `observed_windows`: application name, window title, process path, process ID, window position, window size, first observed time, last observed time, and associated event count.

This data may contain sensitive information. For example, window titles may reveal file names, web page titles, chat contacts, project names, or other private context. Please decide whether to run, retain, or share the database file based on your own risk tolerance.

## Database Location

Horologion currently decides the database path based on the run mode:

| Run mode | Default database location |
| --- | --- |
| Development | `playground/db/horologion.db` under the project root |
| Production | `horologion/horologion.db` under the system user data directory |
| Test | In-memory database `:memory:` |

In production mode, the system user data directory is usually similar to:

- macOS: `~/Library/Application Support/horologion/horologion.db`
- Linux: `~/.local/share/horologion/horologion.db`, or `$XDG_DATA_HOME/horologion/horologion.db`
- Windows: `%APPDATA%\horologion\horologion.db`

The actual path can be viewed in the app settings page. Because built-in cleanup and backup features are not available yet, if the database becomes large, close the app first and then manually move, back up, or delete the corresponding `horologion.db` file.

In development mode, `playground/` is ignored by `.gitignore` to avoid accidentally committing local database files.

## Configuration Status

Configuration support is still experimental. The main environment variables available at this stage are:

| Variable | Description |
| --- | --- |
| `RUN_MODE` | Can be set to `test`, `dev`, `development`, `prod`, or `production`. If unset, debug builds default to Development and release builds default to Production. |
| `DATABASE_PATH` | Specifies the local database file path. |
| `DATABASE_URL` | Currently handled as the database connection value. At this stage, the project mainly targets local DuckDB files. |
| `LOG_LEVEL` | Used as the default log level when `RUST_LOG` is not set. Defaults to `info`. |
| `RUST_LOG` | Rust log filter configuration. Takes precedence over `LOG_LEVEL`. |

Production mode attempts to read the database path from the configuration file, but this part is still unstable. Future versions will reorganize the configuration structure, documentation, and migration strategy.

## Local Development

Recommended environment:

- Node.js and pnpm
- Rust stable
- System dependencies required by Tauri 2
- On macOS, input monitoring and accessibility permissions are required, otherwise input collection or window title collection may not work

Install dependencies:

```bash
pnpm install
```

Start the frontend development server:

```bash
pnpm dev
```

Start the Tauri desktop app:

```bash
pnpm tauri-dev
```

Start the listener separately:

```bash
pnpm listener
```

Build the frontend:

```bash
pnpm build
```

Build the listener sidecar:

```bash
pnpm build:listener
```

Package the Tauri app:

```bash
pnpm tauri build
```

The Tauri build process runs `pnpm build:listener` through `beforeBuildCommand`, builds the listener sidecar, and copies it into `src-tauri/bin/` with the platform-specific sidecar name expected by Tauri.

## Project Structure

```text
.
├── src/              # React frontend
├── src-tauri/        # Tauri main process, command routing, and sidecar management
├── src-listener/     # Keyboard, mouse, and active window listener
├── src-db/           # DuckDB connection, schema, models, and query APIs
├── Cargo.toml        # Rust workspace
├── package.json      # Frontend and Tauri scripts
├── README.md         # English README
└── README-ZH.md      # Chinese README
```

Core module relationships:

- `src-listener` collects input events and active window context.
- `src-tauri` starts the listener sidecar, receives event JSON, and writes it to the database.
- `src-db` provides database initialization, insertion, querying, pagination, and aggregation APIs.
- `src` calls backend APIs through Tauri commands and displays overview, event, window, and settings pages.

## Tech Stack

- Tauri 2
- Rust
- DuckDB
- React 19
- TypeScript
- Vite
- TanStack Query
- Tailwind CSS
- i18next

## Roadmap

- Improve error handling and user-facing failure messages.
- Build a clearer logging strategy, including log levels and log file handling.
- Add database rotation, compression, cleanup, and backup features.
- Stabilize the configuration file format and provide complete configuration documentation.
- Improve cross-platform permission guidance and packaging flow.
- Add more tests, especially for database APIs, Tauri commands, and listener behavior.
- Expand statistics and analysis features while preserving privacy.

## License

This project is open-sourced under the [MIT License](LICENSE).
