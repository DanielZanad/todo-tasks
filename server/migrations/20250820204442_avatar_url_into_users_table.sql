-- Add migration script here
-- Add migration script here
ALTER TABLE users
ADD COLUMN avatar_url VARCHAR(255) NOT NULL;