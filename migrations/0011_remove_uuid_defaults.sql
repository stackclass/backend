-- Migration to remove UUID default values since IDs are generated in application code
-- This allows using UUID v7 from the application layer

-- Remove default values from all tables
ALTER TABLE courses ALTER COLUMN id DROP DEFAULT;
ALTER TABLE extensions ALTER COLUMN id DROP DEFAULT;
ALTER TABLE stages ALTER COLUMN id DROP DEFAULT;
ALTER TABLE user_courses ALTER COLUMN id DROP DEFAULT;
ALTER TABLE user_stages ALTER COLUMN id DROP DEFAULT;

-- Drop the uuid-ossp extension since it's no longer needed
DROP EXTENSION IF EXISTS "uuid-ossp";
