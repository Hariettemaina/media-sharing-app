-- Your SQL goes here
CREATE TABLE IF NOT EXISTS email_address(
    id SERIAL PRIMARY KEY,
    email varchar not null unique,
    verification_code uuid not null,
    verification_code_expires_at timestamp not null,
    verified_at timestamp default null
);
