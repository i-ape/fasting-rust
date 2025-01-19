CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- Nullable<Integer> (auto-generated)
    username TEXT NOT NULL,
    hashed_password TEXT NOT NULL,
    device_id TEXT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NULL
);

CREATE TABLE fasting_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    start_time TIMESTAMP NOT NULL,
    stop_time TIMESTAMP NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE fasting_goals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    goal_duration INTEGER NOT NULL,
    deadline TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

