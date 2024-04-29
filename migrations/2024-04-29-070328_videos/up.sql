-- Your SQL goes here
CREATE TABLE IF NOT EXISTS videos(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR,
    codec_name VARCHAR,
    duration VARCHAR,
    file_path VARCHAR NOT NULL,
    width INTEGER,
    height INTEGER,
    bitrate VARCHAR,
    frame_rate VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
