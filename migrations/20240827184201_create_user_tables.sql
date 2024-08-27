-- Add migration script here
-- Create users table
CREATE TABLE users
(
  id SERIAL PRIMARY KEY,
  username VARCHAR(50) UNIQUE NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add user_id to tasks table
ALTER TABLE tasks ADD user_id INTEGER REFERENCES users(id) NOT NULL;

-- Create index on user_id in tasks table
CREATE INDEX idx_tasks_user_id ON tasks(user_id);
