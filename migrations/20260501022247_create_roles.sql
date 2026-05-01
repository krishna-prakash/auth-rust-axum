-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE "roles" (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    UNIQUE(workspace_id, name)
);

CREATE INDEX idx_roles_workspace_id ON "roles"(workspace_id);

