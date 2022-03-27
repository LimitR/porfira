CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
	id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
	login varchar NOT NULL UNIQUE,
	email varchar NOT NULL UNIQUE,
	password_hash varchar NOT NULL,
	ability varchar null,
	image varchar null,
	-- email_verfied
	-- active
	created_at timestamp not null default current_timestamp,
	updated_at timestamp not null default current_timestamp,
 approved boolean default false
);