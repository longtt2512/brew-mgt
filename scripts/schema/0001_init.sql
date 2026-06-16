-- Harness durable layer schema v1
-- Applied by `harness-cli init`. Version-controlled; harness.db is not.

CREATE TABLE IF NOT EXISTS schema_version (
    version    INTEGER NOT NULL,
    applied_at TEXT    NOT NULL
);

-- Intake classifications (one row per request entering the harness).
CREATE TABLE IF NOT EXISTS intake (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT    NOT NULL,
    kind       TEXT    NOT NULL,   -- new-spec | spec-slice | change-request | ...
    summary    TEXT    NOT NULL,
    lane       TEXT    NOT NULL    -- tiny | normal | high-risk
);

-- Story packets and their proof status (the durable test matrix).
CREATE TABLE IF NOT EXISTS stories (
    id                   TEXT PRIMARY KEY,        -- e.g. US-001
    title                TEXT NOT NULL,
    lane                 TEXT NOT NULL,
    status               TEXT NOT NULL DEFAULT 'proposed',
    unit                 INTEGER NOT NULL DEFAULT 0,  -- 0/1
    integration          INTEGER NOT NULL DEFAULT 0,
    e2e                  INTEGER NOT NULL DEFAULT 0,
    platform             INTEGER NOT NULL DEFAULT 0,
    verify_command       TEXT,
    last_verified_at     TEXT,
    last_verified_result TEXT,                    -- pass | fail
    created_at           TEXT NOT NULL
);

-- Durable decision records (mirror of docs/decisions/*.md).
CREATE TABLE IF NOT EXISTS decisions (
    id         TEXT PRIMARY KEY,                  -- e.g. 0001-brew-execution-boundary
    title      TEXT NOT NULL,
    doc        TEXT NOT NULL,                     -- path to the markdown record
    notes      TEXT,
    created_at TEXT NOT NULL
);

-- Execution traces (one per completed task).
CREATE TABLE IF NOT EXISTS traces (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT NOT NULL,
    summary    TEXT NOT NULL,
    outcome    TEXT NOT NULL,                     -- success | partial | blocked | reverted
    story      TEXT,
    friction   TEXT
);

-- Harness growth backlog (items born from friction).
CREATE TABLE IF NOT EXISTS backlog (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT NOT NULL,
    title      TEXT NOT NULL,
    pain       TEXT NOT NULL,
    risk       TEXT NOT NULL DEFAULT 'normal',    -- tiny | normal | high-risk
    predicted  TEXT,
    outcome    TEXT,
    status     TEXT NOT NULL DEFAULT 'open'       -- open | closed
);
