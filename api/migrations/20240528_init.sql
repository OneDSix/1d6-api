CREATE TABLE IF NOT EXISTS mods (
	id serial PRIMARY KEY,
	name TEXT NOT NULL,
	owner serial NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
	id serial PRIMARY KEY,
	username TEXT UNIQUE NOT NULL,
	password TEXT NOT NULL
);
