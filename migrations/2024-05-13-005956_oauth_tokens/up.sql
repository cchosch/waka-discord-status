-- Your SQL goes here
CREATE TABLE waka_oauth_tokens
(
    id            VARCHAR PRIMARY KEY,
    value         VARCHAR     NOT NULL,
    expires_at    TIMESTAMPTZ NOT NULL,
    refresh_token VARCHAR     NOT NULL,
    uid           VARCHAR     NOT NULL,
    replaced      BOOLEAN     NOT NULL
);

CREATE INDEX waka_expiry ON waka_oauth_tokens (expires_at);
CREATE INDEX waka_replaced ON waka_oauth_tokens (replaced);
