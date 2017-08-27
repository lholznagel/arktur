CREATE TABLE IF NOT EXISTS "blockchain" (
  -- id of the blockchain
  id    UUID        PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  -- name of the chain
  name  VARCHAR(50)                                       NOT NULL,
  -- key for a valid signed block (0000 or abcdef)
  -- the start of a hash must match the given pattern
  -- 4 is quick, 6 takes a lot longer
  signkey  VARCHAR(8)                                     NOT NULL
);
