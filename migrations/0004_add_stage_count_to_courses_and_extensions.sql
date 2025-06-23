-- Add stage_count column to courses and extensions tables

-- Courses table
ALTER TABLE courses
ADD COLUMN stage_count INTEGER NOT NULL DEFAULT 0;

-- Extensions table
ALTER TABLE extensions
ADD COLUMN stage_count INTEGER NOT NULL DEFAULT 0;
