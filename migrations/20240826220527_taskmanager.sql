-- Add migration script here
-- Create tasks table
CREATE TABLE tasks
(
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on created_at for efficient sorting
CREATE INDEX idx_tasks_created_at ON tasks(created_at);
