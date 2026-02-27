-- Add migration script here
ALTER TABLE users ADD created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;