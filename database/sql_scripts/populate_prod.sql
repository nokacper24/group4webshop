BEGIN;

INSERT INTO company (company_name, company_address)
VALUES ('Proflex', 'Sandbergvegen 98 6009 Ålesund');

INSERT INTO app_user (email, pass_hash, company_id, role)
VALUES ('admin_proflex@gmail.com', 'mmmsecurepass', 1, 'admin');

COMMIT;