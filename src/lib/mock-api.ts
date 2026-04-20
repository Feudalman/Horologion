export type AppStatus = {
  listenerRunning: boolean;
  databaseReady: boolean;
  lastEventAt: string;
  runMode: "development" | "production" | "test";
  version: string;
  databasePath: string;
};

export type ActivitySummary = {
  totalEvents: number;
  keyEvents: number;
  buttonEvents: number;
  wheelEvents: number;
  activeWindows: number;
  topApps: Array<{
    appName: string;
    eventCount: number;
    share: number;
  }>;
};

export type InputEventPreview = {
  id: number;
  occurredAt: string;
  kind: "key_press" | "key_release" | "button_press" | "button_release" | "wheel";
  value: string;
  appName: string;
  windowTitle: string;
};

const recentEvents: InputEventPreview[] = [
  {
    id: 1084,
    occurredAt: "2026-04-20T13:58:21.230Z",
    kind: "key_press",
    value: "KeyK",
    appName: "Visual Studio Code",
    windowTitle: "Horologion - monitor.rs",
  },
  {
    id: 1083,
    occurredAt: "2026-04-20T13:58:19.840Z",
    kind: "key_release",
    value: "KeyK",
    appName: "Visual Studio Code",
    windowTitle: "Horologion - monitor.rs",
  },
  {
    id: 1082,
    occurredAt: "2026-04-20T13:57:44.030Z",
    kind: "wheel",
    value: "delta_x:0, delta_y:-1",
    appName: "Arc",
    windowTitle: "DuckDB documentation",
  },
  {
    id: 1081,
    occurredAt: "2026-04-20T13:56:12.930Z",
    kind: "button_press",
    value: "Left",
    appName: "Terminal",
    windowTitle: "cargo check -p listener",
  },
];

const status: AppStatus = {
  listenerRunning: true,
  databaseReady: true,
  lastEventAt: recentEvents[0].occurredAt,
  runMode: "development",
  version: "0.1.0",
  databasePath: "/Users/zirui/projects/arui/Horologion/playground/db/horologion.db",
};

const summary: ActivitySummary = {
  totalEvents: 1284,
  keyEvents: 962,
  buttonEvents: 221,
  wheelEvents: 101,
  activeWindows: 18,
  topApps: [
    { appName: "Visual Studio Code", eventCount: 624, share: 49 },
    { appName: "Terminal", eventCount: 278, share: 22 },
    { appName: "Arc", eventCount: 193, share: 15 },
    { appName: "Finder", eventCount: 91, share: 7 },
    { appName: "Tauri", eventCount: 52, share: 4 },
  ],
};

function delay<T>(value: T) {
  return new Promise<T>((resolve) => {
    window.setTimeout(() => resolve(value), 160);
  });
}

export function getAppStatus() {
  return delay(status);
}

export function getActivitySummary() {
  return delay(summary);
}

export function listRecentEvents() {
  return delay(recentEvents);
}
