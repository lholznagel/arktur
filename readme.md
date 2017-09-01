# rust-blockchain

Sample blockchain written in rust

## Goal
This is a small project to play a little bit with block chains. The goal is NOT to build another crypto currency.
The goal is to explore blockchain and rust :)

## Current concept ideas
- girst let's get started with a prototype -> only local
- Mesh Network
  - In a config file, every other peer is noted with IP and Port
  - Every peer can be connected with multiple peers
  - Example config file
  ``` yaml
  peers:
    - address: "localhost"
      port: 8001
      name: "Peer 1"
    - address: "localhost"
      port: 8002
      name: "Peer 2"
    - address: "localhost"
      port: 8003
      name: "Peer 3"
  ```
  - on startup, every peer is notified that the peer has started
  - the notified peers give that to there peers and those peers give that to there peers and so on
  - see chapter `Peer communication` on how the communication should work
- miners communicate with the peer using an api
- every peer should have its own database and miners
- every peer needs at least one miner (how to identify miners that are not available?)
- at least 3 peers are required
- every peer acts like an api server to the network
- when a new block is created, all peers are notified and start mining
- when a miner finds a result he tells the peer and the peer tells the other miners and all peers
- every peer validates the entry
- TODO: find out how to validate that all said OK

### Peer communication
- every peer send all messages he gets to other peers
- in order to prevent loop messages, all send messages will be saved in a database
- an example entry in JSON:
``` json
{
  "content": "content as JSON object",
  "hash": "SHA256 hash to identify the message (contains all fields of this object)",
  "peer": "Identification of the peer (did not decide how it should be generated)",
  "timestamp": 0
}
```
- every client checks if the message hash already exists in the database
- if yes then don´t send it to any peer
  - because the peer knows the message, all other peers also know the message
  - because the peer already send the message to the other peers