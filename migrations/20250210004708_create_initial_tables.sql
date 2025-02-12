-- Add migration script here
CREATE TABLE todo (
    todo_id uuid PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    create_time timestamptz NOT NULL DEFAULT NOW()
);


