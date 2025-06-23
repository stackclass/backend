-- Add logo column to courses table

ALTER TABLE courses
ADD COLUMN logo TEXT NOT NULL DEFAULT '';
