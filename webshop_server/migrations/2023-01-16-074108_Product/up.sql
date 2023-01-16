-- Your SQL goes here
CREATE TABLE Product (
    product_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    amount_available INT NOT NULL,
    main_image references Image(image_id),
    other_image references ImageProduct(image_id),
)