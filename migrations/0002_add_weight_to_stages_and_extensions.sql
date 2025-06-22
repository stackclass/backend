-- Add weight column to extensions and stages tables

-- Extensions table
ALTER TABLE extensions
ADD COLUMN weight INTEGER NOT NULL DEFAULT 0;

-- Stages table
ALTER TABLE stages
ADD COLUMN weight INTEGER NOT NULL DEFAULT 0;
