CREATE TABLE fasting_goals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    goal_duration INTEGER NOT NULL,
    deadline TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL
);
