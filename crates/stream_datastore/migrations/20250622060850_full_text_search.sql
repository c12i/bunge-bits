-- Add migration script here
-- Date: 2025-06-19
-- Purpose: Add full-text search support to `streams` table and log user search queries
-- NOTE: These changes were applied manually in the database.
-- We make use of `IF NOT EXISTS` to ensure that the migration can be run multiple times without error. ie: in the case of the production database.


-- 1. Add column
ALTER TABLE streams ADD COLUMN IF NOT EXISTS search_vector tsvector;

-- 2. GIN index
CREATE INDEX IF NOT EXISTS streams_search_vector_idx ON streams USING GIN (search_vector);

-- 3. Trigger function
CREATE OR REPLACE FUNCTION update_streams_search_vector()
RETURNS trigger AS $$
BEGIN
  NEW.search_vector := to_tsvector(
    'english',
    coalesce(NEW.title, '') || ' ' || coalesce(NEW.summary_md, '')
  );
  RETURN NEW;
END
$$ LANGUAGE plpgsql;

-- 4. Trigger
-- dropping the trigger for the environments where it already exists.
DROP TRIGGER IF EXISTS streams_search_vector_trigger ON streams;

CREATE TRIGGER streams_search_vector_trigger
BEFORE INSERT OR UPDATE ON streams
FOR EACH ROW EXECUTE FUNCTION update_streams_search_vector();

-- 5. Backfill for existing rows
UPDATE streams
SET search_vector = to_tsvector('english', coalesce(title, '') || ' ' || coalesce(summary_md, ''))
WHERE search_vector IS NULL;

-- 6. Search query log table
CREATE TABLE  IF NOT EXISTS search_queries (
  id SERIAL PRIMARY KEY,
  query TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


CREATE INDEX IF NOT EXISTS idx_search_queries_created_at ON search_queries(created_at);
