-- Your SQL goes here

CREATE TABLE users
(
    id              SERIAL PRIMARY KEY,
    name            character varying(255) NOT NULL,
    password        character varying(255) NOT NULL,
    email           character varying(255) NOT NULL
);

CREATE TABLE projects
(
    id          SERIAL PRIMARY KEY,
    name        character varying(255) NOT NULL,
    user_id     integer                NOT NULL
);

CREATE TABLE tasks
(
    id              SERIAL PRIMARY KEY,
    content         character varying(1024) NOT NULL,
    status          integer NOT NULL,
    created_time    bigint  NOT NULL,
    updated_time    bigint  NOT NULL,
    project_id      integer NOT NULL
);

ALTER TABLE ONLY projects
    ADD CONSTRAINT fk_projects_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON UPDATE RESTRICT ON DELETE RESTRICT;

ALTER TABLE ONLY tasks
    ADD CONSTRAINT fk_tasks_project_id FOREIGN KEY (project_id) REFERENCES projects(id) ON UPDATE RESTRICT ON DELETE RESTRICT;

