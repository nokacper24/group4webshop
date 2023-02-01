--postgreSQL simple table for test
--product where name is the primary key
CREATE TABLE product (
    product_name TEXT NOT NULL,
    product_price REAL NOT NULL,

    PRIMARY KEY (product_name)
);