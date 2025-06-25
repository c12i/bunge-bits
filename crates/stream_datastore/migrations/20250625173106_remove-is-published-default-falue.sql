-- Add migration script here
ALTER TABLE streams ALTER COLUMN is_published SET DEFAULT TRUE;
