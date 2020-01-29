-- Your SQL goes here
CREATE TABLE post_tags(
	post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
	tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
	CONSTRAINT post_tags_pk PRIMARY KEY(post_id, tag_id)
);
