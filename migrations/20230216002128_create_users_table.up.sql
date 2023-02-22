-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id VARCHAR(255) UNIQUE,
    name VARCHAR(50) NOT NULL,
    avatar_file_name VARCHAR(50),
    email VARCHAR(255) NOT NULL UNIQUE,
    hashed_password VARCHAR(255) NOT NULL
)