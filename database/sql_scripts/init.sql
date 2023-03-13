BEGIN;

CREATE TYPE role_enum AS ENUM ('admin', 'company_it_head', 'company_it', 'default');

CREATE TABLE company (
    company_id SERIAL PRIMARY KEY,
    company_name TEXT NOT NULL,
    company_address TEXT NOT NULL
);

CREATE TABLE register_company_user (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    exp_date timestamptz NOT NULL,
    company_id INT NOT NULL,
    FOREIGN KEY (company_id) REFERENCES company(company_id)
);

CREATE TABLE app_user (
    user_id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    pass_hash TEXT NOT NULL,
    company_id INT NOT NULL,
    role role_enum NOT NULL,
    FOREIGN KEY (company_id) REFERENCES company(company_id)
);

CREATE TABLE register_user (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    exp_date timestamptz NOT NULL
);

CREATE TABLE invite_user (
    id TEXT PRIMARY KEY,
   
    /* Optional foreign keys to partial user and partial company user */
    user_id INT,
    company_user_id INT,

    FOREIGN KEY (user_id) REFERENCES register_user(id),
    FOREIGN KEY (company_user_id) REFERENCES register_company_user(id)
);

CREATE TABLE cookies (
    id SERIAL PRIMARY KEY,
    cookie TEXT NOT NULL,
    exp timestamptz NOT NULL,
    user_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES app_user(user_id)
);

CREATE TABLE product (
    product_id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    price_per_user REAL NOT NULL,
    short_description TEXT NOT NULL,
    main_image TEXT NOT NULL,
    available BOOLEAN NOT NULL
);

CREATE TABLE license (
    license_id SERIAL PRIMARY KEY,
    valid BOOLEAN NOT NULL,
    start_date timestamptz NOT NULL,
    end_date timestamptz NOT NULL,
    amount INT NOT NULL,
    company_id INT NOT NULL,
    product_id TEXT NOT NULL,
    FOREIGN KEY (company_id) REFERENCES company(company_id),
    FOREIGN KEY (product_id) REFERENCES product(product_id)
);

CREATE TABLE user_license (
    license_id INT NOT NULL,
    user_id INT NOT NULL,
    PRIMARY KEY (license_id, user_id),
    FOREIGN KEY (license_id) REFERENCES license(license_id),
    FOREIGN KEY (user_id) REFERENCES app_user(user_id)
);

CREATE TABLE category (
    category_id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE product_category (
    product_id TEXT NOT NULL,
    category_id INT NOT NULL,
    PRIMARY KEY (product_id, category_id),
    FOREIGN KEY (product_id) REFERENCES product(product_id),
    FOREIGN KEY (category_id) REFERENCES category(category_id)
);

CREATE TABLE testimonial (
    testimonial_id SERIAL PRIMARY KEY,
    author TEXT NOT NULL,
    text TEXT NOT NULL,
    author_pic TEXT NOT NULL,
    product_id TEXT NOT NULL,
    FOREIGN KEY (product_id) REFERENCES product(product_id)
);

CREATE TABLE product_image (
    image_id SERIAL PRIMARY KEY,
    image_path TEXT NOT NULL,
    alt_text TEXT NOT NULL
);

CREATE TABLE product_text (
    text_id SERIAL PRIMARY KEY,
    text_title TEXT NOT NULL,
    paragraph TEXT NOT NULL
);

CREATE TABLE description_component (
    component_id SERIAL PRIMARY KEY,
    priority INT NOT NULL,
    product_id TEXT NOT NULL,
    image_id INT,
    text_id INT,
    FOREIGN KEY (product_id) REFERENCES product(product_id),
    FOREIGN KEY (image_id) REFERENCES product_image(image_id),
    FOREIGN KEY (text_id) REFERENCES product_text(text_id),
    UNIQUE (product_id, priority) DEFERRABLE INITIALLY DEFERRED, -- Ensure unique priority for components of a product
    CHECK ((image_id IS NOT NULL AND text_id IS NULL) OR (image_id IS NULL AND text_id IS NOT NULL)) -- Ensure that it has EITHER an image or a text, not both
);

CREATE OR REPLACE FUNCTION set_description_component_priority()
RETURNS TRIGGER AS $$
DECLARE
  max_priority INTEGER;
BEGIN
  SELECT MAX(priority) INTO max_priority FROM description_component WHERE product_id = NEW.product_id;
  IF max_priority IS NULL THEN
    max_priority := 0;
  END IF;
  NEW.priority := max_priority + 1;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_description_component_priority_trigger
  BEFORE INSERT ON description_component
  FOR EACH ROW
  EXECUTE FUNCTION set_description_component_priority();

CREATE OR REPLACE FUNCTION delete_description_component()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.image_id IS NOT NULL THEN
        DELETE FROM product_image WHERE image_id = OLD.image_id;
    ELSIF OLD.text_id IS NOT NULL THEN
        DELETE FROM product_text WHERE text_id = OLD.text_id;
    END IF;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER delete_description_component_trigger
AFTER DELETE ON description_component
FOR EACH ROW EXECUTE FUNCTION delete_description_component();

COMMIT;