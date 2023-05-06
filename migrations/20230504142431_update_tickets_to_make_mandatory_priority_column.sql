-- Update tickets table to make mandatory `priority` column.
ALTER TABLE tickets
ALTER COLUMN priority
SET NOT NULL;