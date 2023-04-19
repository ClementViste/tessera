-- Update tickets table to add `is_open` column.
ALTER TABLE tickets
ADD COLUMN is_open BOOLEAN NULL;