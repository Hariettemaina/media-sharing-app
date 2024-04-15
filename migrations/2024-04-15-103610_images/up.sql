-- Your SQL goes here
create table if not exists images(
    id serial primary key,
    name varchar not null,
    file_path varchar not null,
    description varchar,
    exif_data varchar,
    format varchar not null,
    size integer not null,
    width integer not null,
    height integer not null,
    created_at timestamp not null default now(),
    deleted_at timestamp
);