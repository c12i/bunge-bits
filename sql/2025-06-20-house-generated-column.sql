-- Date: 2025-06-20
-- Purpose: Add a house generated column
-- NOTE: These changes were applied manually in the database.
--       This file exists for documentation and reference purposes only.
-- TODO: Integrate these changes into a proper sqlx migration once migration support is added.

ALTER TABLE streams
ADD COLUMN house TEXT GENERATED ALWAYS AS (
  CASE
    WHEN title ILIKE '%national assembly%' AND title ILIKE '%senate%' THEN 'all'
    WHEN title ILIKE '%national assembly%' THEN 'national assembly'
    WHEN title ILIKE '%senate%' THEN 'senate'
    ELSE 'unspecified'
  END
) STORED;
