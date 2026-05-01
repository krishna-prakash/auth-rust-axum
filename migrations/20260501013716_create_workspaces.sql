-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE "workspaces" (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    owner_user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    invite_code TEXT UNIQUE NOT NULL DEFAULT encode(gen_random_bytes(16), 'hex'),
    is_default BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(owner_user_id, name)
);

CREATE INDEX idx_workspaces_owner_user_id ON "workspaces"(owner_user_id);
