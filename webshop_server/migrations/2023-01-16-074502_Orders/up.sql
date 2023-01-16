-- Your SQL goes here
CREATE TABLE Orders (
    order_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    Foreign Key (user_id) REFERENCES Users(user_id),
    order_date DATE NOT NULL,
    status_id INT NOT NULL,
    Foreign Key (status_id) REFERENCES Status(status_id)
)