-- This file should undo anything in `up.sql`

DELETE FROM timeslots CASCADE;

DELETE FROM doctors CASCADE;

DELETE FROM users CASCADE;

DELETE FROM evaluations CASCADE;
