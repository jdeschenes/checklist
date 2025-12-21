-- Add migration script here
BEGIN;
  CREATE TYPE todo_visibility AS ENUM ('public', 'private');

  ALTER TABLE todo ADD COLUMN visibility todo_visibility NOT NULL DEFAULT 'public';

  ALTER TABLE todo ALTER COLUMN visibility DROP DEFAULT;
COMMIT;
