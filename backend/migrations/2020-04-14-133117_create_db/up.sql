-- Your SQL goes here

CREATE TABLE doctors
(
    id              SERIAL PRIMARY KEY,
    name            character varying(255) NOT NULL,
    email           character varying(255) NOT NULL,
    sex             boolean                NOT NULL,
    year            integer                NOT NULL,
    education       integer                NOT NULL,
    occupation      integer                NOT NULL,
    occupation2     character varying(255) NOT NULL,
    occupation_spec character varying(255) NOT NULL,
    certificate     integer                NOT NULL,
    modalitet       integer                NOT NULL,
    modalitet2      character varying(255) NOT NULL,
    phone           character varying(32)  NOT NULL,
    zoom            character varying(32)  NOT NULL,
    confirmed       boolean                NOT NULL,
    dateUpdated     bigint                 NOT NULL,
    user_id         integer                NOT NULL
);

CREATE TABLE evaluations
(
    id          SERIAL PRIMARY KEY,
    sex         integer                 NOT NULL,
    age         integer                 NOT NULL,
    problem     character varying(1024) NOT NULL,
    help        character varying(1024) NOT NULL,
    success     boolean                 NOT NULL,
    forward     boolean                 NOT NULL,
    comment     character varying(1024) NOT NULL,
    timeslot_id integer                 NOT NULL
);

CREATE TABLE timeslots
(
    id        SERIAL PRIMARY KEY,
    status    integer NOT NULL,
    datetime  bigint  NOT NULL,
    doctor_id integer NOT NULL
);

CREATE TABLE users
(
    id       SERIAL PRIMARY KEY,
    name     character varying(128) NOT NULL,
    password character varying(255) NOT NULL,
    role     integer                NOT NULL
);

ALTER TABLE ONLY doctors
    ADD CONSTRAINT fk_doctors_user_id_id FOREIGN KEY (user_id) REFERENCES users(id) ON UPDATE RESTRICT ON DELETE RESTRICT;

ALTER TABLE ONLY evaluations
    ADD CONSTRAINT fk_evaluations_timeslot_id_id FOREIGN KEY (timeslot_id) REFERENCES timeslots(id) ON UPDATE RESTRICT ON DELETE RESTRICT;

ALTER TABLE ONLY timeslots
    ADD CONSTRAINT fk_timeslots_doctor_id_id FOREIGN KEY (doctor_id) REFERENCES doctors(id) ON UPDATE RESTRICT ON DELETE RESTRICT;
