-- Diesel migration to create user_auth table

-- Table to store unique users
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY, -- Unique ID, auto-increment
    accounts TEXT[] NOT NULL, -- List of accounts
    signature TEXT NOT NULL, -- Signature
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Created at
    refresh_token TEXT NOT NULL, -- Refresh token for JWT
    last_login TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Last login
);
