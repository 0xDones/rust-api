-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
	id serial PRIMARY KEY,
	username VARCHAR ( 50 ) UNIQUE NOT NULL,
	email VARCHAR ( 255 ) UNIQUE NOT NULL,
	created_at TIMESTAMP
);
