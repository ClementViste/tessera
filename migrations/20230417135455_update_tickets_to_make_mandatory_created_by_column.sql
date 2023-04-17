-- Update tickets table to make mandatory `created_by` column.
ALTER TABLE tickets
ALTER COLUMN created_by
SET NOT NULL;