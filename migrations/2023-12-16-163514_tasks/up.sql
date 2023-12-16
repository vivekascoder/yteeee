-- Your SQL goes here
CREATE TABLE tasks (
    id  VARCHAR(36) PRIMARY KEY,
    youtube_url VARCHAR(100) NOT NULL,
    status VARCHAR(100) NOT NULL
);