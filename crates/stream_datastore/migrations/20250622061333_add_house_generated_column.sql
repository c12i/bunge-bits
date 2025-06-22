-- Add migration script here
ALTER TABLE streams
ADD COLUMN IF NOT EXISTS house TEXT GENERATED ALWAYS AS (
  CASE
    WHEN title ILIKE '%national assembly%' AND title ILIKE '%senate%' THEN 'all'
    WHEN title ILIKE '%national assembly%' THEN 'national assembly'
    WHEN title ILIKE '%senate%' THEN 'senate'
    ELSE 'unspecified'
  END
) STORED;
