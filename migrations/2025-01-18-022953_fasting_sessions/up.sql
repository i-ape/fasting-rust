CREATE TABLE fasting_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,      -- Non-nullable primary key
    user_id INTEGER NOT NULL,                  -- Foreign key to `users` table
    start_time TIMESTAMP NOT NULL,             -- Start time of the fasting session
    stop_time TIMESTAMP NULL,                  -- Optional stop time
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Auto-generated creation timestamp
);
