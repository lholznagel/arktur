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

Adds new data to the next block.
After sending it to one peer, it is send to all other.

### Payload

- unique_key - For this message. Used to validate that a peer does not get the same message mutliple times
- content - Content for the next block

## BlockGen

Starts the generation of the next block.
The hash of the block contains the index, timestamp, content and the nonce.

### Payload

- index - Index of the block
- timestamp - Timestamp the generation started
- prev - Hash of the previous block
- sign_key - Sign key the new hash must match, for example: The first 4 chars must be `0000`
- content - Content of the block

## BlockFound

After every peer send back the new hash and most say that it is ok, they send back all infromation in order to save it.

### Payload

- index - Index of the block
- timestamp - Timestamp the generation started
- nonce - Nonce of the hash
- prev - Hash of the previous block
- hash - Hash of the block
- content - Content of the block

## HashVal

As soon as a new block is found, the peer sends a message to all other peers to validate the hash.
Every peer should then use the data to generate the hash.

### Payload

- index - Index of the block
- timestamp - Timestamp the generation started
- prev - Hash of the previous block
- nonce - Nonce of the hash
- content - Content of the block

## HashValAck

Sends back the validated hash. ot the peer that first found the hash.

### Payload

- index - Index of the block
- hash - Hash of the generated block