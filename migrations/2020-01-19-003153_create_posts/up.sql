-- Your SQL goes here
CREATE TABLE posts (
	id SERIAL PRIMARY KEY,
	title VARCHAR(255) NOT NULL,
	body text NOT NULL
)
