DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS notes;

CREATE TABLE users (
    user_id INT PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE notes (
    note_id INT PRIMARY KEY,
    note VARCHAR NOT NULL
);

INSERT INTO users (user_id, username, password) VALUES (1, 'username', 'password');

INSERT INTO notes (note_id, note) VALUES (1, 'Meme');
INSERT INTO notes (note_id, note) VALUES (2, 'Do the dishes');
