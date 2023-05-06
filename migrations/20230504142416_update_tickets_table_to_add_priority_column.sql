-- Update tickets table to add `priority` column.
ALTER TABLE tickets
ADD COLUMN priority TEXT NULL;