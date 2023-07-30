-- Add migration script here

CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY NOT NULL,
    project_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (project_id) REFERENCES Project(id)
)