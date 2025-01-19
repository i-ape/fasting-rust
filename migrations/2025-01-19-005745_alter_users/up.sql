-- Check if the 'users' table exists; if it doesn't, create it.
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    hashed_password TEXT NOT NULL,
    device_id TEXT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NULL
);
