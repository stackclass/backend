-- Migration for adding unique constraints to user course and stage tables
-- Ensures no duplicate enrollments or stage progress records

-- Add unique constraint to user_courses table
ALTER TABLE user_courses
ADD CONSTRAINT unique_user_course UNIQUE (user_id, course_id);

-- Add unique constraint to user_stages table
ALTER TABLE user_stages
ADD CONSTRAINT unique_user_stage UNIQUE (user_course_id, stage_id);
