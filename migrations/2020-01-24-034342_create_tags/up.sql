-- Your SQL goes here
CREATE TABLE tags(
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX name_idx ON tags(name);_
