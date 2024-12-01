CREATE TABLE users_old (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    hashed_password TEXT NOT NULL
);

INSERT INTO users_old (id, username, hashed_password)
SELECT id, username, hashed_password
FROM users;

DROP TABLE users;

ALTER TABLE users_old RENAME TO users;
