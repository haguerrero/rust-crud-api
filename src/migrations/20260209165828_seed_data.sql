-- Add migration script here
-- Departments
INSERT INTO departments (name)
VALUES
 ('Engineering'),
 ('Sales'),
 ('Marketing'),
 ('Finance'),
 ('HR');

-- Users (ejemplo simple, luego lo haremos masivo desde Rust)
INSERT INTO users (id, email, password_hash)
VALUES
 (UUID_TO_BIN(UUID()), 'admin@example.com', 'hashed_password');
