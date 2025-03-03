CREATE TABLE IF NOT EXISTS fasting_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL,
    start_time TIMESTAMP NOT NULL,
    stop_time TIMESTAMP NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    goal_id INTEGER NULL,  -- ✅ Links to `fasting_goals`

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,  -- ✅ Ensures user deletion removes fasting records
    FOREIGN KEY (goal_id) REFERENCES fasting_goals (id) ON DELETE SET NULL  -- ✅ If goal is deleted, fasting event remains
);