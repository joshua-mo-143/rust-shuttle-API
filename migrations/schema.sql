DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS notes;

CREATE TABLE users (
    user_id INT GENERATED ALWAYS AS IDENTITY,
    PRIMARY KEY(user_id),
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
);

CREATE TABLE notes (
    note_id INT GENERATED ALWAYS AS IDENTITY,
    PRIMARY KEY(note_id),
    note VARCHAR(255) NOT NULL,

    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES (user_id)
);

INSERT INTO users (username, password)
    VALUES ('username', 'password');

INSERT INTO notes (user_id, note)    
    VALUES (1, 'Meme'), (1, 'Do the dishes');