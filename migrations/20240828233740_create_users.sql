-- Add migration script here
-- Create tasks table
CREATE TABLE tasks
(
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  user_id INT NOT NULL,
  FOREIGN KEY
  (user_id) REFERENCES users
  (id) ON
  DELETE CASCADE
);

-- Create index on user_id for better performance
CREATE INDEX idx_tasks_user_id ON tasks(user_id);
