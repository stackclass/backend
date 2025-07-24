-- Migration to add test column to user_stages table
-- Stores test result status (passed/failed) for user stage progress

ALTER TABLE user_stages ADD COLUMN test TEXT NOT NULL DEFAULT 'failed';

-- Update existing records to maintain consistency
UPDATE user_stages SET test = 'failed' WHERE test IS NULL;
