DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS notes;

CREATE TABLE IF NOT EXISTS users (
    user_id serial PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS notes (
    note_id serial PRIMARY KEY,
    note VARCHAR NOT NULL,
    user_id integer,

    CONSTRAINT FK_notes_users FOREIGN KEY (user_id) REFERENCES users(user_id)
);

INSERT INTO users (user_id, username, password) VALUES (1, 'username', 'password');

INSERT INTO notes (user_id, note) VALUES (1, 'Meme');
INSERT INTO notes (user_id, note) VALUES (1, 'Do the dishes');