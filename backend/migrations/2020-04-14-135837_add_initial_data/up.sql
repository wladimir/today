-- Your SQL goes here

INSERT INTO users
	VALUES (1, 'a@a.com', 'admin', 99),
	       (2, 'd@d.com', 'doc', 10);

INSERT INTO doctors (id, name, email, sex, year, education, occupation, occupation2, occupation_spec, certificate, modalitet, modalitet2, phone, zoom, confirmed, dateUpdated, user_id)
	VALUES (1, 'Petar Petrović', 'Petar Petrovic@dot.com', true, 1990, 4, 1, 'occ', 'spec', 0, 2, 'cert', '641294217', 'Z-00-M', false, 1586699282825, 2);

INSERT INTO timeslots (id, status, datetime, doctor_id)
	VALUES (1, 10, 202004131548, 1);
