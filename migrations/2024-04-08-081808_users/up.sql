-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    first_name varchar not null,
    middle_name varchar,
    last_name varchar not null,
    username varchar not null unique,
    email_address varchar not null,
    email_verification_code UUID not null,
    email_verified boolean not null default false,
    email_verification_code_expiry timestamp not null,
    password_hash varchar not null,
    display_name varchar,
    date_of_birth date not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);