-- Your SQL goes here
CREATE TABLE ProductOrder (
    product_id foreign key references Product(product_id),
    order_id foreign key references Order(order_id),
    amount INT NOT NULL,
)