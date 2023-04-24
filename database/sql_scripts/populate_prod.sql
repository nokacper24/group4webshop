BEGIN;

INSERT INTO company (company_name, company_address)
VALUES ('Proflex', 'Sandbergvegen 98 6009 Ålesund');

INSERT INTO app_user (email, pass_hash, company_id, role)
VALUES ('admin_proflex@gmail.com', '$argon2id$v=19$m=4096,t=3,p=1$zGDWbzz553WuAOr9wAscPw$scsw+3Q2pW19Jqz8uXQpzikahS3a8CNubaC1EQH3Fa4', 1, 'admin');

COMMIT;