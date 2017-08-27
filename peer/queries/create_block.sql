CREATE TABLE IF NOT EXISTS "block" (
  -- blockchain the block belongs
  blockchain UUID NOT NULL,
  -- index of the block
  index INTEGER NOT NULL,
  -- manipulator for changing the hash
  nonce INTEGER NOT NULL,
  -- content of the block
  content TEXT NOT NULL,
  -- timestamp
  timestamp BIGINT NOT NULL,
  -- previous hash
  prev VARCHAR(64) NOT NULL,
  -- block hash
  hash VARCHAR(64) NOT NULL
);
