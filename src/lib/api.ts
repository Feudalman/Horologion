import { invoke } from "@tauri-apps/api/core";

export type AppStatus = {
  listenerRunning: boolean;
  databaseReady: boolean;
  lastEventAt: string | null;
  runMode: "development" | "production" | "test";
  version: string;
  databasePath: string | null;
};

export type AppSettings = {
  runMode: "development" | "production" | "test";
  version: string;
  databasePath: string | null;
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

export type InputEventKind =
  | "key_press"
  | "key_release"
  | "button_press"
  | "button_release"
  | "wheel";

export type InputEventPreview = {
  id: number;
  occurredAt: string;
  kind: InputEventKind;
  value: string;
  appName: string;
  windowTitle: string;
};

type BackendAppStatus = {
  listener_running: boolean;
  database_ready: boolean;
  last_event_at: string | null;
  run_mode: AppStatus["runMode"];
  app_version: string;
  database_path: string | null;
};

type BackendAppSettings = {
  run_mode: AppSettings["runMode"];
  app_version: string;
  database_path: string | null;
};

type BackendActivitySummary = {
  total_events: number;
  key_events: number;
  button_events: number;
  wheel_events: number;
  active_windows: number;
  top_apps: Array<{
    app_name: string;
    event_count: number;
    share: number;
  }>;
};

type BackendPaginatedResponse<T> = {
  page: number;
  total: number;
  pages: number;
  list: T[];
};

type BackendInputEventWithWindow = {
  event: {
    event_id: number;
    occurred_at: string;
    kind: InputEventKind;
    value: string;
    delta_x: number | null;
    delta_y: number | null;
  };
  window: {
    app_name: string;
    title: string;
  } | null;
};

export async function getAppStatus() {
  const status = await invoke<BackendAppStatus>("get_app_status");

  return {
    listenerRunning: status.listener_running,
    databaseReady: status.database_ready,
    lastEventAt: status.last_event_at,
    runMode: status.run_mode,
    version: status.app_version,
    databasePath: status.database_path,
  } satisfies AppStatus;
}

export async function getAppSettings() {
  const settings = await invoke<BackendAppSettings>("get_app_settings");

  return {
    runMode: settings.run_mode,
    version: settings.app_version,
    databasePath: settings.database_path,
  } satisfies AppSettings;
}

export async function getActivitySummary() {
  const summary = await invoke<BackendActivitySummary>("get_activity_summary");

  return {
    totalEvents: summary.total_events,
    keyEvents: summary.key_events,
    buttonEvents: summary.button_events,
    wheelEvents: summary.wheel_events,
    activeWindows: summary.active_windows,
    topApps: summary.top_apps.map((app) => ({
      appName: app.app_name,
      eventCount: app.event_count,
      share: app.share,
    })),
  } satisfies ActivitySummary;
}

export async function listRecentEvents() {
  const response = await invoke<BackendPaginatedResponse<BackendInputEventWithWindow>>(
    "list_input_events",
    {
      query: {
        page: 1,
        size: 10,
        sort_by: "occurred_at",
        sort_direction: "desc",
      },
    },
  );

  return response.list.map(({ event, window }) => ({
    id: event.event_id,
    occurredAt: event.occurred_at,
    kind: event.kind,
    value: formatEventValue(event),
    appName: window?.app_name ?? "Unknown",
    windowTitle: window?.title ?? "",
  })) satisfies InputEventPreview[];
}

function formatEventValue(event: BackendInputEventWithWindow["event"]) {
  if (event.kind !== "wheel") {
    return event.value;
  }

  const deltaX = event.delta_x ?? 0;
  const deltaY = event.delta_y ?? 0;

  return event.value || `delta_x:${deltaX}, delta_y:${deltaY}`;
}
