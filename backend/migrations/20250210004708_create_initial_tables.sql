CREATE OR REPLACE FUNCTION update_time_trigger() RETURNS trigger 
    LANGUAGE plpgsql AS
$$
BEGIN
    NEW.update_time = NOW();
    RETURN NEW;
END;
$$;

CREATE TABLE users (
    user_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    email TEXT NOT NULL UNIQUE,
    create_time timestamptz NOT NULL DEFAULT NOW(),
    update_time timestamptz NOT NULL DEFAULT NOW()
);

CREATE TRIGGER trig_users_update_time BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE PROCEDURE update_time_trigger();

-- Add migration script here
CREATE TABLE todo (
    todo_id uuid PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (user_id) ON DELETE CASCADE,
    name TEXT NOT NULL UNIQUE,
    create_time timestamptz NOT NULL DEFAULT NOW(),
    update_time timestamptz NOT NULL DEFAULT NOW()
);

-- Add recurring template table for storing recurrence rules
CREATE TABLE recurring_template (
    template_id uuid PRIMARY KEY,
    todo_id uuid NOT NULL REFERENCES todo (todo_id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users (user_id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    recurrence_period INTERVAL NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NULL,
    last_generated_date DATE NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    create_time timestamptz NOT NULL DEFAULT NOW(),
    update_time timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE todo_item (
    todo_item_id uuid PRIMARY KEY,
    todo_id uuid NOT NULL REFERENCES todo (todo_id) ON DELETE CASCADE,
    recurring_template_id uuid NULL REFERENCES recurring_template (template_id) ON DELETE SET NULL,
    user_id INTEGER NOT NULL REFERENCES users (user_id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    is_complete BOOLEAN NOT NULL DEFAULT FALSE,
    due_date date NOT NULL DEFAULT NOW(),
    complete_time timestamptz NULL,
    create_time timestamptz NOT NULL DEFAULT NOW(),
    update_time timestamptz NOT NULL DEFAULT NOW()
);

CREATE TRIGGER trig_todo_update_time BEFORE UPDATE ON todo
    FOR EACH ROW EXECUTE PROCEDURE update_time_trigger();

CREATE TRIGGER trig_todo_item_update_time BEFORE UPDATE ON todo_item
    FOR EACH ROW EXECUTE PROCEDURE update_time_trigger();

-- Add update trigger for recurring_template
CREATE TRIGGER trig_recurring_template_update_time BEFORE UPDATE ON recurring_template
    FOR EACH ROW EXECUTE PROCEDURE update_time_trigger();

-- Add index for efficient queries on active templates
CREATE INDEX idx_recurring_template_active ON recurring_template (is_active, last_generated_date) WHERE is_active = TRUE;
CREATE INDEX idx_todo_item_recurring_template_id ON todo_item (recurring_template_id) WHERE recurring_template_id IS NOT NULL;

CREATE INDEX idx_users_email ON users (email);


CREATE INDEX idx_todo_user_id ON todo (user_id);
CREATE INDEX idx_todo_item_user_id ON todo_item (user_id);
CREATE INDEX idx_recurring_template_user_id ON recurring_template (user_id);
