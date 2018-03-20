# Block Types

## GetBlocks

Requests a list of all blocks a peer knows.
In order to get all information about a block, send a `GetBlock` request containing the hash.

### Payload

None

## GetBlocksAck

Gets a list of all blocks a peer knows.
Only the filename of the block is send.

### Payload

Single array of Strings - Every string represents the last 16 chars of hash

## GetBlock

Requests a specific block from a peer.
The other peer sends back all information about the requested block.

### Payload

Single string - Last 16 chars of the block

## GetBlockAck

Contains all information about the requested block.

### Payload

- filename - last 16 chars of the hash
- index - index in the blockchain
- timestamp - timestamp the block was generated
- nonce - of the block
- prev - hash of the previous block
- hash - hash of the current block
- content - content that is saved in the block

## BlockData

## BlockGen

## BlockFound

## HashVal

## HashValAck