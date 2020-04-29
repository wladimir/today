-- Your SQL goes here

CREATE TABLE users
(
    id              SERIAL PRIMARY KEY,
    username        character varying(255) NOT NULL,
    password        character varying(255) NOT NULL,
    email           character varying(255) NOT NULL
);

CREATE TABLE tasks
(
    id              SERIAL PRIMARY KEY,
    content         character varying(1024) NOT NULL,
    status          integer NOT NULL,
    created_time    bigint  NOT NULL,
    updated_time    bigint  NOT NULL,
    user_id         integer NOT NULL
);

ALTER TABLE ONLY tasks
    ADD CONSTRAINT fk_tasks_project_id FOREIGN KEY (user_id) REFERENCES users(id) ON UPDATE RESTRICT ON DELETE RESTRICT;

