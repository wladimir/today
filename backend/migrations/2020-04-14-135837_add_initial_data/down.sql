-- This file should undo anything in `up.sql`

DELETE FROM tasks CASCADE;

DELETE FROM projects CASCADE;

DELETE FROM users CASCADE;
