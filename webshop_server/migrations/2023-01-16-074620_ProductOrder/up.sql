-- Your SQL goes here
CREATE TABLE ProductOrder (
    product_order_id SERIAL PRIMARY KEY,
    product_id INT NOT NULL,
    Foreign Key (product_id) REFERENCES Product(product_id),
    order_id INT NOT NULL,
    Foreign Key (order_id) REFERENCES Orders(order_id),
    amount INT NOT NULL
);