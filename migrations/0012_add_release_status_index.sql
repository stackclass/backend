-- Migration to add index on courses.release_status for better query performance

-- Add index on release_status column
CREATE INDEX idx_courses_release_status ON courses(release_status);
