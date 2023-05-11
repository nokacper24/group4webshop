BEGIN;

INSERT INTO company (company_name, company_address)
VALUES ('Proflex', 'Sandbergvegen 98 6009 Ålesund');

INSERT INTO app_user (email, pass_hash, company_id, role)
VALUES ('admin_proflex@gmail.com', '$argon2id$v=19$m=4096,t=3,p=1$zGDWbzz553WuAOr9wAscPw$scsw+3Q2pW19Jqz8uXQpzikahS3a8CNubaC1EQH3Fa4', 1, 'admin');

INSERT INTO product ("product_id", "display_name", "price_per_user", "short_description", "main_image", "available") VALUES
('online_scheduling_software',	'Online Scheduling Software',	2000,	'Streamline your scheduling process with Proflex Solutions'' Online Scheduling Software. Automate appointments, manage resources, and track performance effortlessly. Boost productivity and save time today!',	'/resources/images/online_scheduling_software/Oransj_calender_software_on_monitor_ab65cb91-6bb0-4891-896b-1e52813dfd49.png',	'1'),
('proflex_bpa_solutions',	'Proflex BPA Solutions',	5000,	'Automate and optimize your business processes with ProFlex BPA Solutions. Streamline operations, save time and money, and focus on business growth with our secure and efficient software.',	'/resources/images/proflex_bpa_solutions/Oransj_automation_software_on_monitor_f69ffed8-f8db-4aa3-b416-57c66d238db9.png',	'1'),
('proflex_tax_solutions',	'ProFlex Tax Solutions',	2000,	'Streamline your tax preparation and filing process with ProFlex Tax Solutions. Enter data quickly and accurately, automate workflows, and focus on your business while ensuring the security and efficiency of your tax-related data.',	'/resources/images/proflex_tax_solutions/Product-4.jpg',	'1');

INSERT INTO product_image ("image_path", "alt_text") VALUES
('/resources/images/online_scheduling_software/Product-1.jpg',	'mobile calender interface'),
('/resources/images/proflex_bpa_solutions/Product-2.png',	'workflow planner interface'),
('/resources/images/proflex_tax_solutions/PRoduct-6.jpg',	'tax prediction interface');

INSERT INTO product_text ("text_title", "paragraph") VALUES
('Efficient Scheduling Made Simple',	'Introducing Proflex Solutions'' Online Scheduling Software! Automate appointments, manage resources, and boost productivity effortlessly. Save time and streamline your scheduling process today.'),
('Powerful Calendar Management and Reporting',	'Take control of your team''s schedule with Proflex Solutions'' Online Scheduling Software. Track performance, analyze data, and optimize efficiency with our robust calendar management and reporting capabilities.'),
('Streamline Your Business Operations',	'Discover the best scheduling software on the market! Proflex Solutions'' Online Scheduling Software is designed for businesses of all sizes. Say goodbye to errors and time-consuming tasks, and hello to a streamlined and productive workflow. Try it out today!'),
('Streamline and Optimize Operations',	'ProFlex BPA Solutions empowers businesses with streamlined and optimized operations. Design and implement automated workflows effortlessly, saving time and maximizing efficiency.'),
('Secure, Reliable, and Efficient',	'Take control of your business processes with ProFlex BPA Solutions. Our secure and reliable software ensures data safety, allowing you to focus on business growth and success.'),
('Streamline Tax Preparation and Filing',	'ProFlex Tax Solutions simplifies and optimizes the tax preparation and filing process. Enter data accurately, automate workflows, and maximize efficiency to achieve tax success effortlessly.'),
('Secure, Reliable, and Efficient Tax Solution',	'Take charge of your tax preparation with ProFlex Tax Solutions. Our secure and reliable software ensures data protection, allowing you to focus on your business while maximizing tax-related efficiency and success.');

INSERT INTO description_component ("priority", "product_id", "full_width", "image_id", "text_id") VALUES
(1,	'online_scheduling_software',	'0',	NULL,	1),
(2,	'online_scheduling_software',	'0',	NULL,	2),
(3,	'online_scheduling_software',	'0',	1,	NULL),
(4,	'online_scheduling_software',	'0',	NULL,	3),
(1,	'proflex_bpa_solutions',	'1',	NULL,	4),
(2,	'proflex_bpa_solutions',	'0',	2,	NULL),
(3,	'proflex_bpa_solutions',	'0',	NULL,	5),
(1,	'proflex_tax_solutions',	'1',	NULL,	6),
(2,	'proflex_tax_solutions',	'0',	NULL,	7),
(3,	'proflex_tax_solutions',	'0',	3,	NULL);

INSERT INTO testimonial ("author", "text", "author_pic", "product_id") VALUES
('Sarah Thompson',	'ProFlex BPA Solutions transformed our business processes! With its intuitive interface and automated workflows, we saved time and optimized operations. Highly recommended!',	'/resources/images/proflex_bpa_solutions/sarahthompson.jpg',	'proflex_bpa_solutions'),
('John Anderson',	'ProFlex Tax Solutions made tax season a breeze! Quick data entry, automated workflows, and top-notch security. It simplified our process and maximized our tax success.',	'/resources/images/proflex_tax_solutions/JohnAnderson.jpg',	'proflex_tax_solutions');

COMMIT;