-- Your SQL goes here
CREATE TABLE Users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    role_id INT NOT NULL,
    Foreign Key (role_id) REFERENCES Role(role_id)
)