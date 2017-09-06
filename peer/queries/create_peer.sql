CREATE TABLE IF NOT EXISTS "peers" (
  -- name of the peer
  name          TEXT PRIMARY KEY  NOT NULL,
  -- timestamp when the peer as registered itself
  registered_at BIGINT            NOT NULL,
  -- timestamp when the last message was send fromt this peer
  last_seen     BIGINT            NOT NULL
);
