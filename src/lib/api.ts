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

export type DatabaseFileSize = {
  sizeBytes: number | null;
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

export type SortDirection = "asc" | "desc";

export type PaginatedResponse<T> = {
  page: number;
  total: number;
  pages: number;
  list: T[];
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
  windowId: number | null;
};

export type InputEventSortBy =
  | "occurred_at"
  | "event_id"
  | "kind"
  | "value"
  | "app_name";

export type InputEventQuery = {
  page?: number;
  size?: number;
  kind?: InputEventKind;
  search?: string;
  sortBy?: InputEventSortBy;
  sortDirection?: SortDirection;
  windowId?: number;
};

export type InputEventListItem = InputEventPreview & {
  collectorName: string;
  collectorVersion: string;
};

export type InputEventDetail = {
  id: number;
  occurredAt: string;
  kind: InputEventKind;
  value: string;
  deltaX: number | null;
  deltaY: number | null;
  windowId: number | null;
  rawEvent: string | null;
  rawWindow: string | null;
  collectorName: string;
  collectorVersion: string;
  createdAt: string;
};

export type ObservedWindowSortBy =
  | "last_seen_at"
  | "first_seen_at"
  | "window_id"
  | "app_name"
  | "event_count";

export type ObservedWindowQuery = {
  page?: number;
  size?: number;
  search?: string;
  sortBy?: ObservedWindowSortBy;
  sortDirection?: SortDirection;
};

export type ObservedWindowListItem = {
  id: number;
  appName: string;
  processPath: string | null;
  processId: number | null;
  title: string;
  x: number | null;
  y: number | null;
  width: number | null;
  height: number | null;
  firstSeenAt: string;
  lastSeenAt: string;
  eventCount: number;
  contextHash: string;
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

type BackendDatabaseFileSize = {
  size_bytes: number | null;
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
    window_id: number | null;
    collector_name: string;
    collector_version: string;
  };
  window: {
    app_name: string;
    title: string;
  } | null;
};

type BackendInputEventRecord = {
  event_id: number;
  occurred_at: string;
  kind: InputEventKind;
  value: string;
  delta_x: number | null;
  delta_y: number | null;
  window_id: number | null;
  raw_event: string | null;
  raw_window: string | null;
  collector_name: string;
  collector_version: string;
  created_at: string;
};

type BackendObservedWindowRecord = {
  window_id: number;
  app_name: string;
  process_path: string | null;
  process_id: number | null;
  title: string;
  x: number | null;
  y: number | null;
  width: number | null;
  height: number | null;
  first_seen_at: string;
  last_seen_at: string;
  event_count: number;
  context_hash: string;
};

export async function getAppStatus() {
  const status = await invoke<BackendAppStatus>("get_app_status");

  return mapAppStatus(status);
}

export async function startListener() {
  const status = await invoke<BackendAppStatus>("start_listener");

  return mapAppStatus(status);
}

export async function stopListener() {
  const status = await invoke<BackendAppStatus>("stop_listener");

  return mapAppStatus(status);
}

export async function getAppSettings() {
  const settings = await invoke<BackendAppSettings>("get_app_settings");

  return {
    runMode: settings.run_mode,
    version: settings.app_version,
    databasePath: settings.database_path,
  } satisfies AppSettings;
}

function mapAppStatus(status: BackendAppStatus): AppStatus {
  return {
    listenerRunning: status.listener_running,
    databaseReady: status.database_ready,
    lastEventAt: status.last_event_at,
    runMode: status.run_mode,
    version: status.app_version,
    databasePath: status.database_path,
  };
}

export async function getDatabaseFileSize() {
  const fileSize = await invoke<BackendDatabaseFileSize>("get_database_file_size");

  return {
    sizeBytes: fileSize.size_bytes,
  } satisfies DatabaseFileSize;
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
  const response = await listInputEvents({
    page: 1,
    size: 10,
    sortBy: "occurred_at",
    sortDirection: "desc",
  });

  return response.list.map((event) => ({
    id: event.id,
    occurredAt: event.occurredAt,
    kind: event.kind,
    value: event.value,
    appName: event.appName,
    windowTitle: event.windowTitle,
    windowId: event.windowId,
  }));
}

export async function listInputEvents(query: InputEventQuery = {}) {
  const response = await invoke<BackendPaginatedResponse<BackendInputEventWithWindow>>(
    "list_input_events",
    {
      query: {
        page: query.page,
        size: query.size,
        kind: query.kind,
        search: emptyToUndefined(query.search),
        window_id: query.windowId,
        sort_by: query.sortBy,
        sort_direction: query.sortDirection,
      },
    },
  );

  return {
    ...response,
    list: response.list.map(mapInputEvent),
  } satisfies PaginatedResponse<InputEventListItem>;
}

export async function getInputEvent(eventId: number) {
  const event = await invoke<BackendInputEventRecord | null>("get_input_event", {
    eventId,
  });

  return event ? mapInputEventDetail(event) : null;
}

export async function listObservedWindows(query: ObservedWindowQuery = {}) {
  const response = await invoke<BackendPaginatedResponse<BackendObservedWindowRecord>>(
    "list_observed_windows",
    {
      query: {
        page: query.page,
        size: query.size,
        search: emptyToUndefined(query.search),
        sort_by: query.sortBy,
        sort_direction: query.sortDirection,
      },
    },
  );

  return {
    ...response,
    list: response.list.map(mapObservedWindow),
  } satisfies PaginatedResponse<ObservedWindowListItem>;
}

export async function getObservedWindow(windowId: number) {
  const window = await invoke<BackendObservedWindowRecord | null>(
    "get_observed_window",
    {
      windowId,
    },
  );

  return window ? mapObservedWindow(window) : null;
}

function mapInputEvent({
  event,
  window,
}: BackendInputEventWithWindow): InputEventListItem {
  return {
    id: event.event_id,
    occurredAt: event.occurred_at,
    kind: event.kind,
    value: formatEventValue(event),
    appName: window?.app_name ?? "Unknown",
    windowTitle: window?.title ?? "",
    collectorName: event.collector_name,
    collectorVersion: event.collector_version,
    windowId: event.window_id,
  };
}

function mapObservedWindow(window: BackendObservedWindowRecord): ObservedWindowListItem {
  return {
    id: window.window_id,
    appName: window.app_name,
    processPath: window.process_path,
    processId: window.process_id,
    title: window.title,
    x: window.x,
    y: window.y,
    width: window.width,
    height: window.height,
    firstSeenAt: window.first_seen_at,
    lastSeenAt: window.last_seen_at,
    eventCount: window.event_count,
    contextHash: window.context_hash,
  };
}

function mapInputEventDetail(event: BackendInputEventRecord): InputEventDetail {
  return {
    id: event.event_id,
    occurredAt: event.occurred_at,
    kind: event.kind,
    value: formatEventValue(event),
    deltaX: event.delta_x,
    deltaY: event.delta_y,
    windowId: event.window_id,
    rawEvent: event.raw_event,
    rawWindow: event.raw_window,
    collectorName: event.collector_name,
    collectorVersion: event.collector_version,
    createdAt: event.created_at,
  };
}

function emptyToUndefined(value: string | undefined) {
  const trimmed = value?.trim();
  return trimmed ? trimmed : undefined;
}

function formatEventValue(event: BackendInputEventWithWindow["event"]) {
  if (event.kind !== "wheel") {
    return event.value;
  }

  const deltaX = event.delta_x ?? 0;
  const deltaY = event.delta_y ?? 0;

  return event.value || `delta_x:${deltaX}, delta_y:${deltaY}`;
}
