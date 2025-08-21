-- Add migration script here
DROP TABLE IF EXISTS avatars;
CREATE TABLE avatars (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) NOT NULL,
    file_key VARCHAR(255) NOT NULL,
    created_at TIMESTAMP default CURRENT_TIMESTAMP
);