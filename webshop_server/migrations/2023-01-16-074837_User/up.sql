-- Your SQL goes here
CREATE TABLE User {
    user_id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role primary key references Role(role_id),
}