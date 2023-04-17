-- Update tickets table to backfill `created_by` column.
UPDATE tickets
SET created_by = 'admin'
WHERE created_by IS NULL;