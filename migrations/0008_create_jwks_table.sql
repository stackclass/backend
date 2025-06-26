
-- Migration for creating the JWKS (JSON Web Key Set) table
-- Stores public and private keys for JWT signing and verification

CREATE TABLE "jwks" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "public_key" TEXT NOT NULL,
    "private_key" TEXT NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index for performance
CREATE INDEX "jwks_id_idx" ON "jwks" ("id");
