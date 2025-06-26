-- Migration for user course enrollment and stage progress tables
-- Compatible with the UserCourseModel and UserStageModel definitions

-- User course enrollment table (tracks user enrollment in courses)
CREATE TABLE user_courses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    current_stage_id UUID REFERENCES stages(id) ON DELETE SET NULL,
    completed_stage_count INTEGER NOT NULL DEFAULT 0,
    proficiency TEXT NOT NULL,
    cadence TEXT NOT NULL,
    accountability BOOLEAN NOT NULL DEFAULT false,
    activated BOOLEAN NOT NULL DEFAULT false
);

-- User stage progress table (tracks user progress in stages)
CREATE TABLE user_stages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_course_id UUID NOT NULL REFERENCES user_courses(id) ON DELETE CASCADE,
    stage_id UUID NOT NULL REFERENCES stages(id) ON DELETE CASCADE,
    status TEXT NOT NULL CHECK (status IN ('in_progress', 'completed')),
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE
);

-- Indexes for performance
CREATE INDEX idx_user_courses_user_id ON user_courses(user_id);
CREATE INDEX idx_user_courses_course_id ON user_courses(course_id);
CREATE INDEX idx_user_stages_user_course_id ON user_stages(user_course_id);
CREATE INDEX idx_user_stages_stage_id ON user_stages(stage_id);
CREATE INDEX idx_user_stages_status ON user_stages(status);
