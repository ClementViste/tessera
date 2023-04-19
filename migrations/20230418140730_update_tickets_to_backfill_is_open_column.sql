-- Update tickets table to backfill `is_open` column.
UPDATE tickets
SET is_open = TRUE
WHERE is_open IS NULL;