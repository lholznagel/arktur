# rust-blockchain

Sample blockchain written in rust

## Current concept idea

- API Server that stands between the Web frontend and all Peers
- For communication between miners and peers RabbitMQ is used
- At least 3 Peers are required, all have their own database
- Every peer has hat least one miner attached
- When a new block is created, all peers are notfied and start mining
- As soon as a miner finds a result, the peer distributes it to all other
- All peers validate the token
- Because every peer knows the other peers they can check if all peers agree to the result
- If all agree the final result is insereted into the database