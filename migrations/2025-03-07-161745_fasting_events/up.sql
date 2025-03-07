PRAGMA foreign_keys=OFF;

CREATE TABLE fasting_events_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER NOT NULL,
    start_time TIMESTAMP NOT NULL,
    stop_time TIMESTAMP NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    goal_id INTEGER NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (goal_id) REFERENCES fasting_goals(id) ON DELETE SET NULL
);

INSERT INTO fasting_events_new (id, user_id, start_time, stop_time, created_at)
SELECT id, user_id, start_time, stop_time, created_at FROM fasting_events;

DROP TABLE fasting_events;

ALTER TABLE fasting_events_new RENAME TO fasting_events;

PRAGMA foreign_keys=ON;
