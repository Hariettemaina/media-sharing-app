-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    first_name varchar not null,
    middle_name varchar,
    last_name varchar not null,
    username varchar not null unique,
    user_email integer references email_address(id) not null,
    password_hash varchar not null,
    display_name varchar,
    date_of_birth date not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);