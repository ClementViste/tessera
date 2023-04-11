-- Update tickets table to add `created_at` column.
ALTER TABLE tickets
ADD COLUMN created_at timestamptz NULL;