-- Update tickets table to make mandatory `created_at` column.
ALTER TABLE tickets
ALTER COLUMN created_at
SET NOT NULL;