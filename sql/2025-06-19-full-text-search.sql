-- Date: 2025-06-19
-- Purpose: Add full-text search support to `streams` table and log user search queries
-- NOTE: These changes were applied manually in the database.
--       This file exists for documentation and reference purposes only.
-- TODO: Integrate these changes into a proper sqlx migration once migration support is added.

-- 1. Add column
ALTER TABLE streams ADD COLUMN search_vector tsvector;

-- 2. GIN index
CREATE INDEX streams_search_vector_idx ON streams USING GIN (search_vector);

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
CREATE TRIGGER streams_search_vector_trigger
BEFORE INSERT OR UPDATE ON streams
FOR EACH ROW EXECUTE FUNCTION update_streams_search_vector();

-- 5. Backfill for existing rows
UPDATE streams
SET search_vector = to_tsvector('english', coalesce(title, '') || ' ' || coalesce(summary_md, ''))
WHERE search_vector IS NULL;

-- 6. Search query log table
CREATE TABLE search_queries (
  id SERIAL PRIMARY KEY,
  query TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 7. Optional index
CREATE INDEX idx_search_queries_created_at ON search_queries(created_at);
