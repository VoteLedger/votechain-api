-- Diesel migration to create user_auth table

-- Table to store unique users
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY, -- Unique ID, auto-increment
    primary_account TEXT NOT NULL, -- Primary account
    refresh_token TEXT NOT NULL, -- Refresh token for JWT
    last_login TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Last login
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Created at
);
