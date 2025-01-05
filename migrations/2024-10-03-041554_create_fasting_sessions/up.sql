CREATE TABLE IF NOT EXISTS fasting_sessions (
    id TEXT PRIMARY KEY,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NULL
);