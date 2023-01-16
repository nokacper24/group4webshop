-- Your SQL goes here
CREATE TABLE Order (
    order_id SERIAL PRIMARY KEY,
    user_id foreign key references User(user_id),
    order_date DATE NOT NULL,
    status foreign key references status(status_id),
)