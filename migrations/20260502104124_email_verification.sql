-- Add migration script here
CREATE TABLE email_verifications (
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    token UUID UNIQUE NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (user_id)
);


