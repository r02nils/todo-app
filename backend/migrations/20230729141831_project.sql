-- Add migration script here
CREATE TABLE IF NOT EXISTS project (
    id INTEGER PRIMARY KEY NOT NULL,
    description TEXT NOT NULL
)