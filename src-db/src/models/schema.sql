-- 为窗口快照和输入事件生成自增主键。
CREATE SEQUENCE IF NOT EXISTS observed_windows_id_seq START 1;
CREATE SEQUENCE IF NOT EXISTS input_events_id_seq START 1;

-- 记录监听事件发生时的活动窗口快照。
CREATE TABLE IF NOT EXISTS observed_windows (
    window_id BIGINT PRIMARY KEY DEFAULT nextval('observed_windows_id_seq'),
    app_name TEXT NOT NULL,
    process_path TEXT,
    process_id UBIGINT,
    title TEXT NOT NULL,
    x DOUBLE,
    y DOUBLE,
    width DOUBLE,
    height DOUBLE,
    first_seen_at TIMESTAMPTZ NOT NULL,
    last_seen_at TIMESTAMPTZ NOT NULL,
    event_count UBIGINT NOT NULL DEFAULT 0,
    context_hash TEXT NOT NULL UNIQUE
);

-- 记录键盘、鼠标按钮和滚轮等原始输入事件。
CREATE TABLE IF NOT EXISTS input_events (
    event_id BIGINT PRIMARY KEY DEFAULT nextval('input_events_id_seq'),
    occurred_at TIMESTAMPTZ NOT NULL,
    event_kind TEXT NOT NULL CHECK (
        event_kind IN (
            'key_press',
            'key_release',
            'button_press',
            'button_release',
            'wheel'
        )
    ),
    event_value TEXT NOT NULL,
    delta_x DOUBLE,
    delta_y DOUBLE,
    window_id BIGINT REFERENCES observed_windows(window_id),
    raw_event TEXT,
    raw_window TEXT,
    collector_name TEXT NOT NULL DEFAULT 'listener',
    collector_version TEXT NOT NULL DEFAULT 'unknown',
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);

-- 按常见查询维度建立索引，便于后续按时间、事件类型和窗口聚合分析。
CREATE INDEX IF NOT EXISTS idx_input_events_occurred_at
    ON input_events (occurred_at);

CREATE INDEX IF NOT EXISTS idx_input_events_kind
    ON input_events (event_kind);

CREATE INDEX IF NOT EXISTS idx_input_events_window
    ON input_events (window_id);

CREATE INDEX IF NOT EXISTS idx_input_events_collector
    ON input_events (collector_name, collector_version);

CREATE INDEX IF NOT EXISTS idx_observed_windows_app
    ON observed_windows (app_name);
