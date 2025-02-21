-- Add migration script here
CREATE TABLE todo (
    todo_id uuid PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    create_time timestamptz NOT NULL DEFAULT NOW(),
    update_time timestamptz NOT NULL DEFAULT NOW()
);


CREATE TABLE todo_item (
    todo_item_id uuid PRIMARY KEY,
    todo_id uuid NOT NULL REFERENCES todo (todo_id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    is_complete BOOLEAN NOT NULL DEFAULT FALSE,
    due_date date NOT NULL DEFAULT NOW(),
    complete_time timestamptz NULL,
    create_time timestamptz NOT NULL DEFAULT NOW(),
    update_time timestamptz NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION update_time_trigger() RETURNS trigger 
    LANGUAGE plpgsql AS
$$
BEGIN
    NEW.update_time = NOW();
    RETURN NEW;
END;
$$;

CREATE TRIGGER trig_todo_update_time BEFORE UPDATE ON todo
    FOR EACH ROW EXECUTE PROCEDURE update_time_trigger();

CREATE TRIGGER trig_todo_update_time BEFORE UPDATE ON todo_item
    FOR EACH ROW EXECUTE PROCEDURE update_time_trigger();

