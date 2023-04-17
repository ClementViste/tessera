-- Update tickets table to add `created_by` column.
ALTER TABLE tickets
ADD COLUMN created_by TEXT NULL REFERENCES users (username);