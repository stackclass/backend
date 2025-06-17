-- Initial database schema for CodeCraft platform
-- Compatible with all model definitions and changed files

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Timestamp update function for all tables
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Courses table (core entity)
CREATE TABLE courses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    short_name TEXT NOT NULL,
    release_status TEXT NOT NULL CHECK (release_status IN ('alpha', 'beta', 'live')),
    description TEXT NOT NULL,
    summary TEXT NOT NULL,
    repository TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TRIGGER update_courses_timestamp
BEFORE UPDATE ON courses
FOR EACH ROW EXECUTE FUNCTION update_timestamp();

-- Extensions table (1:N with courses)
CREATE TABLE extensions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    slug TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE (course_id, slug)
);

CREATE TRIGGER update_extensions_timestamp
BEFORE UPDATE ON extensions
FOR EACH ROW EXECUTE FUNCTION update_timestamp();

-- Stages table (1:N with courses/extensions)
CREATE TABLE stages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    extension_id UUID REFERENCES extensions(id) ON DELETE CASCADE,
    slug TEXT NOT NULL,
    name TEXT NOT NULL,
    difficulty TEXT NOT NULL CHECK (difficulty IN ('very_easy', 'easy', 'medium', 'hard')),
    description TEXT NOT NULL,
    instruction TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE (course_id, slug),
    CONSTRAINT unique_extension_slug UNIQUE (extension_id, slug)
);

CREATE TRIGGER update_stages_timestamp
BEFORE UPDATE ON stages
FOR EACH ROW EXECUTE FUNCTION update_timestamp();

-- Solutions table (1:1 with stages)
CREATE TABLE solutions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stage_id UUID UNIQUE NOT NULL REFERENCES stages(id) ON DELETE CASCADE,
    explanation TEXT NOT NULL,
    patches JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TRIGGER update_solutions_timestamp
BEFORE UPDATE ON solutions
FOR EACH ROW EXECUTE FUNCTION update_timestamp();

-- Indexes for performance
CREATE INDEX idx_courses_slug ON courses(slug);
CREATE INDEX idx_extensions_course ON extensions(course_id);
CREATE INDEX idx_extensions_slug ON extensions(slug);
CREATE INDEX idx_solutions_stage ON solutions(stage_id);
CREATE INDEX idx_stages_course ON stages(course_id);
CREATE INDEX idx_stages_extension ON stages(extension_id);
CREATE INDEX idx_stages_slug ON stages(slug);
