-- Migration to drop the solutions table and add a solution field to the stages table

-- Drop the solutions table and its associated trigger
DROP TRIGGER IF EXISTS update_solutions_timestamp ON solutions;
DROP TABLE IF EXISTS solutions;

-- Drop the index for the solutions table
DROP INDEX IF EXISTS idx_solutions_stage;

-- Add the solution column to the stages table
ALTER TABLE stages
ADD COLUMN solution TEXT;

-- Update the existing stages to set the solution field to NULL
UPDATE stages
SET solution = NULL;
