BEGIN;

INSERT INTO company (company_name, company_address)
VALUES ('Proflex', 'sandbergvegen 98 6009 Ålesund'),
        ('Enterprise Solutions Inc.', 'Cupertino, California'),
        ('Business Solutions LLC', 'Redmond, Washington');

INSERT INTO app_user (email, pass_hash, company_id, role)
VALUES ('admin_proflex@gmail.com', 'mmmsecurepass', 1, 'admin'),
        ('es_it_head@enterprisesolutions.com', 'pass', 2, 'company_it_head'),
        ('it_manager@enterprisesolutions.com', 'pass', 2, 'company_it_head'),
        ('it_manager_2@enterprisesolutions.com', 'pass', 2, 'company_it_head'),
        ('bs_it_head@businesssolutions.com', 'pass', 3, 'company_it_head');

INSERT INTO cookies (cookie, exp, user_id)
VALUES ('cookie1', '2026-12-31', 2),
       ('cookie2', '2027-06-30', 3),
       ('admin_cookie', '4000-06-30', 1);

INSERT INTO product (product_id, display_name, price_per_user, short_description, main_image, available)
VALUES ('time_management_software', 'Time Management System', 999.99, 'A comprehensive time management solution for enterprises.', 'time_management.jpg', true),
       ('3d_modelling_software', '3D Modelling', 29.99, 'A fast and easy-to-learn software for 3D modelling, with extensive tools for perfecting details.', '3d_modelling.jpg', true),
       ('tax_and_accounting_software', 'Tax and Accounting Software', 899.99, 'A software solution for tax and accounting needs of enterprises.', 'tax_accounting.jpg', true),
       ('online_scheduling_software', 'Online Scheduling Software', 200, 'Our software is designed to help you effectively manage and streamline the scheduling process. It helps you automate the scheduling process, allowing you to save time and reduce errors.', 'online_scheduling.jpg', true),
       ('bpa_solutions', 'BPA Solutions', 500, 'Automation of processes. It has an intuitive user interface, and automates workflows.', 'bpa.jpg', true);

INSERT INTO license (valid, start_date, end_date, amount, company_id, product_id)
VALUES (false, '2022-01-01', '2022-12-31', 100, 2, 'time_management_software'),
       (true, '2023-07-01', '2023-06-30', 50, 3, 'tax_and_accounting_software'),
       (true, '2023-01-01', '2023-12-31', 50, 2, 'tax_and_accounting_software'),
       (true, '2023-01-01', '2023-12-31', 15, 2, '3d_modelling_software');

INSERT INTO user_license (license_id, user_id)
VALUES (1, 2),
       (2, 2),
       (2, 3);

INSERT INTO category (name, description)
VALUES ('Enterprise Software', 'Software solutions for enterprises'),
       ('Financial Software', 'Financial software solutions for enterprises');

INSERT INTO product_category (product_id, category_id)
VALUES ('time_management_software', 1),
       ('tax_and_accounting_software', 2);

INSERT INTO testimonial (author, text, author_pic, product_id)
VALUES ('Jane Doe', 'I am extremely impressed with the time management software provided by the company. It has greatly improved the efficiency of my team.', 'jane.jpg', 'time_management_software'),
       ('John Smith', 'The tax software is a lifesaver. The software makes tax season a breeze and saves me so much time.', 'john.jpg', 'tax_and_accounting_software'),
       ('Stephanie Smith', 'This has enabled us to streamline our operations and save time. The product is easy to use and the customer service is top-notch.', 'stephanie_smith.jpg', 'time_management_software');

INSERT INTO product_image (image_path, alt_text)
VALUES ('time_management_software.jpg', 'Time Management Software Interface'),
       ('tax_software.jpg', 'Tax Software Interface');

INSERT INTO product_text (text_title, paragraph)
VALUES ('Time Management Software Features', 'Our time management software offers a range of features such as task scheduling, time tracking, and reporting. These features help organizations streamline their work processes and improve productivity.'),
       ('Tax Software Benefits', 'Our tax software is designed to simplify the tax preparation process for businesses. The software eliminates the need for manual calculations and reduces the risk of errors. It also saves time and reduces the stress associated with tax season.');

INSERT INTO description_component (product_id, full_width, image_id, text_id)
VALUES ('time_management_software', false, 1, null),
       ('time_management_software', false, null, 1),
       ('tax_and_accounting_software', false, 2, null),
       ('tax_and_accounting_software', false, null, 2);

COMMIT;