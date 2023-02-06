BEGIN;

INSERT INTO company (company_id, company_name, company_address)
VALUES (1, 'Enterprise Solutions Inc.', 'Cupertino, California'),
       (2, 'Business Solutions LLC', 'Redmond, Washington');

INSERT INTO register_company_user (id, key, email, exp_date, company_id)
VALUES (1, 'abc123', 'es_hr@enterprisesolutions.com', '2021-06-01', 1),
       (2, 'def456', 'bs_hr@businesssolutions.com', '2022-01-01', 2);

INSERT INTO app_user (user_id, email, pass_hash, company_id, role)
VALUES (1, 'es_it_head@enterprisesolutions.com', 'pass', 1, 'company_it_head'),
       (2, 'bs_it_head@businesssolutions.com', 'pass', 2, 'company_it_head');

INSERT INTO register_user (id, key, email, exp_date)
VALUES (1, 'ghi789', 'jane_enterprise@gmail.com', '2021-12-01'),
       (2, 'jkl101', 'john_business@gmail.com', '2022-03-01');

INSERT INTO cookies (id, cookie, exp, user_id)
VALUES (1, 'cookie1', '2021-12-31', 1),
       (2, 'cookie2', '2022-06-30', 2);

INSERT INTO product (product_id, display_name, price_per_user, short_description, main_image)
VALUES ('time_managment_software', 'Time Management System', 999.99, 'A comprehensive time management solution for enterprises', 'time_management.jpg'),
       ('tax_and_accounting_software', 'Tax and Accounting Software', 899.99, 'A software solution for tax and accounting needs of enterprises', 'tax_accounting.jpg');

INSERT INTO license (license_id, valid, start_date, end_date, amount, company_id, product_id)
VALUES (1, true, '2021-01-01', '2021-12-31', 10000, 1, 'time_managment_software'),
       (2, true, '2021-07-01', '2022-06-30', 5000, 2, 'tax_and_accounting_software');

INSERT INTO user_license (license_id, user_id)
VALUES (1, 1),
       (2, 2);

INSERT INTO category (id, name, description)
VALUES (1, 'Enterprise Software', 'Software solutions for enterprises'),
       (2, 'Financial Software', 'Financial software solutions for enterprises');

INSERT INTO product_category (product_id, category_id)
VALUES ('time_managment_software', 1),
       ('tax_and_accounting_software', 2);

INSERT INTO testemonial (testemonial_id, author, text, author_pic, product_id)
VALUES (1, 'Jane Doe', 'I am extremely impressed with the time management software provided by the company. It has greatly improved the efficiency of my team.', 'jane.jpg', 'time_managment_software'),
       (2, 'John Smith', 'The tax software is a lifesaver. The software makes tax season a breeze and saves me so much time.', 'john.jpg', 'tax_and_accounting_software');

INSERT INTO product_image (image_id, image_path, alt_text)
VALUES (1, 'time_management_software.jpg', 'Time Management Software Interface'),
       (2, 'tax_software.jpg', 'Tax Software Interface');

INSERT INTO product_text (text_id, text_title, paragraph)
VALUES (1, 'Time Management Software Features', 'Our time management software offers a range of features such as task scheduling, time tracking, and reporting. These features help organizations streamline their work processes and improve productivity.'),
       (2, 'Tax Software Benefits', 'Our tax software is designed to simplify the tax preparation process for businesses. The software eliminates the need for manual calculations and reduces the risk of errors. It also saves time and reduces the stress associated with tax season.');

INSERT INTO description_component (component_id, priority, product_id, image_id, text_id)
VALUES (1, 1, 'time_managment_software', 1, null),
       (2, 2, 'time_managment_software', null, 1),
       (3, 1, 'tax_and_accounting_software', 2, null),
       (4, 2, 'tax_and_accounting_software', null, 2);

COMMIT;