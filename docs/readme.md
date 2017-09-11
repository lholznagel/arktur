## Creating a new user in the database

- Login with a superadmin
- For testing the username and password are the same
- Run the follwing commands:

``` sql
CREATE USER "<USERNAME>" PASSWORD '<USERNAME>';
```

``` sql
CREATE DATABASE "<USERNAME>" OWNER "<USERNAME>";
```

- Login with the superadmin into the new created database
- Run the follwing script:
``` sql
CREATE EXTENSION pgcrypto;
```

- After that create all needed tables with:
``` sql
CREATE TABLE IF NOT EXISTS "block" (
  -- blockchain the block belongs
  blockchain  UUID          NOT NULL,
  -- index of the block
  index       INTEGER       NOT NULL,
  -- manipulator for changing the hash
  nonce       INTEGER       NOT NULL,
  -- content of the block
  content     TEXT          NOT NULL,
  -- timestamp
  timestamp   BIGINT        NOT NULL,
  -- previous hash
  prev        VARCHAR(64)   NOT NULL,
  -- block hash
  hash        VARCHAR(64)   NOT NULL
);
```

``` sql
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
```

``` sql
CREATE TABLE IF NOT EXISTS "peers" (
  -- address of the peer (TODO check the max size for the string)
  address           VARCHAR(26)               NOT NULL,
  -- name of the peer
  name              TEXT PRIMARY KEY          NOT NULL,
  -- port of the peer
  port              INTEGER                   NOT NULL,
  -- unique id for the peer
  unique_id         UUID                      NOT NULL,
  -- timestamp when the peer as registered itself
  registered_at     BIGINT                    NOT NULL,
  -- timestamp when the last message was send fromt this peer
  last_seen         BIGINT                    NOT NULL,
  -- true if this peer is directly connected and should be notified on change
  -- example: On a change we notify on peer this peer notifes two others
  notify_on_change  BOOLEAN DEFAULT false     NOT NULL
);
```

- last give the new user permission to select, update and insert data into the tables:
``` sql
GRANT SELECT, UPDATE, INSERT ON peers, block, blockchain TO "<USERNAME>";
```