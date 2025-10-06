-- Add migration script here
ALTER TABLE tasks
ADD COLUMN task_date TIMESTAMP NOT NULL;