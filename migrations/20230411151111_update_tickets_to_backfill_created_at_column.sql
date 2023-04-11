-- Update tickets table to backfill `created_at` column.
UPDATE tickets
SET created_at = '2023-4-11T00:00:00.000000000Z'
WHERE created_at IS NULL;