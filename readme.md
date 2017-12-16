# rust-blockchain

Sample blockchain written in rust

## Goal
This is a small project to play a little bit with block chains. The goal is NOT to build another crypto currency.
The goal is to explore blockchain and rust :)

## Sub projects
The main project consits of mutliple sub projects. Each project is specialized for one thing.

Name | Description
-- | --
connection_manager | Contains the hole puncher and sends all blocks to the connected peers.
file | Handles files actions. For example saving the current connected peers or storing the block data
logging | Small crate for logging. Contains some macros for logging across all crates
network | Contains a UDP-Builder and handles UDP-Connections
peer | Generates the hashes for a block
protocol | Crate for parsing the used protocol

## License
This project is dual licensed under Apache 2.0 and MIT. Please see the license files for more information.