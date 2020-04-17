-- Your SQL goes here

INSERT INTO users(id, name, password, email)
	VALUES (1, 'test', '', 'test@test.com');

INSERT INTO projects(id, name, user_id)
	VALUES (1, 'READING LIST', 1);

INSERT INTO tasks(id, content, status, created_time, updated_time, project_id)
	VALUES (1, 'Moby Dick quotes https://www.goodreads.com/work/quotes/2409320-moby-dick-or-the-whale', 0, 1587162317, 1587162317, 1);
