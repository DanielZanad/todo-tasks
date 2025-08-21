-- Add migration script here
ALTER TABLE avatars
ADD COLUMN mime_type VARCHAR(255) NOT NULL;