-- Update tickets table to make mandatory `is_open` column.
ALTER TABLE tickets
ALTER COLUMN is_open
SET NOT NULL;