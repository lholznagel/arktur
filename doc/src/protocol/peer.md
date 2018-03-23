# Peer Types

## GetPeers

Requests an list of all peers a peer knows.

### Payload

None

## GetPeersAck

Response to GetPeers.
Contains a list of ip addresses that represent the known peers.

### Payload

- peers - Stringlist containing all peer ips.

## Register

Registers a peer at another peer or a hole puncher.

### Payload

None

## RegisterAck

- peers - Stringlist containing all peer ips.