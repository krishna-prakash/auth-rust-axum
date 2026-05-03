-- Add migration script here
ALTER TABLE users RENAME COLUMN udpated_at TO updated_at;
