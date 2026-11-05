ALTER TABLE public.users
    DROP COLUMN auth_version;

CREATE TABLE public.revoked_jwts (
    jti TEXT PRIMARY KEY,
    expires_at BIGINT NOT NULL
);

CREATE INDEX revoked_jwts_expires_at_idx
    ON public.revoked_jwts (expires_at);
