-- Migration to add timezone support to timestamp columns
-- This fixes the issue where tokens expire immediately when server and database
-- are in different timezones (see better-auth issue #3461)

-- Users table
ALTER TABLE "users"
    ALTER COLUMN "created_at" TYPE TIMESTAMP WITH TIME ZONE
        USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN "updated_at" TYPE TIMESTAMP WITH TIME ZONE
        USING updated_at AT TIME ZONE 'UTC';

-- Sessions table
ALTER TABLE "sessions"
    ALTER COLUMN "expires_at" TYPE TIMESTAMP WITH TIME ZONE
        USING expires_at AT TIME ZONE 'UTC',
    ALTER COLUMN "created_at" TYPE TIMESTAMP WITH TIME ZONE
        USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN "updated_at" TYPE TIMESTAMP WITH TIME ZONE
        USING updated_at AT TIME ZONE 'UTC';

-- Accounts table
ALTER TABLE "accounts"
    ALTER COLUMN "access_token_expires_at" TYPE TIMESTAMP WITH TIME ZONE
        USING access_token_expires_at AT TIME ZONE 'UTC',
    ALTER COLUMN "refresh_token_expires_at" TYPE TIMESTAMP WITH TIME ZONE
        USING refresh_token_expires_at AT TIME ZONE 'UTC',
    ALTER COLUMN "created_at" TYPE TIMESTAMP WITH TIME ZONE
        USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN "updated_at" TYPE TIMESTAMP WITH TIME ZONE
        USING updated_at AT TIME ZONE 'UTC';

-- Verifications table
ALTER TABLE "verifications"
    ALTER COLUMN "expires_at" TYPE TIMESTAMP WITH TIME ZONE
        USING expires_at AT TIME ZONE 'UTC',
    ALTER COLUMN "created_at" TYPE TIMESTAMP WITH TIME ZONE
        USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN "updated_at" TYPE TIMESTAMP WITH TIME ZONE
        USING updated_at AT TIME ZONE 'UTC';
