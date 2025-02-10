-- Add migration script here
CREATE TABLE todo (
    todo_id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    create_time timestamptz NOT NULL DEFAULT NOW()
);


