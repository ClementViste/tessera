-- Update tickets table to backfill `priority` column.
UPDATE tickets
SET priority = 'medium'
WHERE priority IS NULL;