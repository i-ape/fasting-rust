CREATE TABLE fasting_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,          -- Unique ID for the fasting event
    user_id INTEGER NOT NULL,                      -- Foreign key referencing the users table
    start_time TIMESTAMP NOT NULL,                 -- Start time of the fasting event
    stop_time TIMESTAMP NULL,                      -- Optional stop time (null if ongoing)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Timestamp when the event was created
    FOREIGN KEY (user_id) REFERENCES users (id)    -- Enforce referential integrity
);
