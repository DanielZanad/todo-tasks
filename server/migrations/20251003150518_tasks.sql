-- Add migration script here
-- Add migration script here
DROP TABLE IF EXISTS tasks;
CREATE TYPE t_status AS ENUM ('ToStart', 'Started', 'Completed');
CREATE TABLE tasks(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) NOT NULL,
    content VARCHAR(255) NOT NULL,
    tasks_status t_status NOT NULL,
    created_at TIMESTAMP default CURRENT_TIMESTAMP
);