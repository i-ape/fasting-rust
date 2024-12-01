CREATE TABLE users_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    hashed_password TEXT NOT NULL,
    device_id TEXT UNIQUE
);

INSERT INTO users_new (id, username, hashed_password)
SELECT id, username, hashed_password
FROM users;

DROP TABLE users;

ALTER TABLE users_new RENAME TO users;
