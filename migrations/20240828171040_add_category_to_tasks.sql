-- Add migration script here
-- In a new migration file (e.g., 2023XXXXXXXX_add_category_to_tasks.sql)

ALTER TABLE tasks
ADD COLUMN category VARCHAR
(50) NOT NULL DEFAULT 'Work';
