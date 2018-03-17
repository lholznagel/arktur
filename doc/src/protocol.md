# Protocol

## Header Structure

```
 00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
| Version               | Type                  |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
| Checksum                                      |
|                                               |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                                               |
//                                             //
//                Payload                      //
//                                             //
|                                               |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
```

- `Version` - Protocol version, currently always 1
- `Type` - Type of the message, see Type chapter.
- `Checksum` - 32 Bit crc checksum, containing the version, type and payload
- `Payload` - Payload is dependent on the `Type`.

### Example Byte Array

`[1, 1, 40, 19, 197, 47, 0]`

- `1` -> Version
- `1` -> Type
- `40 19 197 47` -> Checksum
- `0` -> Payload

## Types

The Type field determines the payload.
Besides that also the application decides what to do with the incoming message.
In the next chapters an overview of all currently avaialable Types are listed and described.

| Range     | Type                                         |
|-----------|----------------------------------------------|
| 000 - 063 | Everything that does not match anything else |
| 064 - 127 | Everything that has to do with peers         |
| 128 - 191 | Everything that has to do with blocks        |
| 192 - 254 | For future use                               |
|       255 | Not a valid command                          |

## Type Codes

List of all types.

| Type        | Code | Description                                    |
|-------------|------|------------------------------------------------|
| Ping        | 000  | Sends a simple ping event                      |
| Pong        | 001  | After a ping send a pong                       |
| Punsh       | 002  | Hole puncher punshes the hole                  |
| GetPeers    | 064  | Requests a list of peers                       |
| GetPeersAck | 065  | Sends back the list of peers                   |
| Register    | 066  | Register at a hole puncher or a peer           |
| RegisterAck | 067  | Acknowledge the register, contains other peers |
| GetBlocks   | 128  | Requests a list of blocks                      |
| GetBlockAck | 129  | Sends back the list of blocks                  |
| GetBlock    | 130  | Requests all data of a single block            |
| GetBlockAck | 131  | Sends back all block informations              |
| BlockData   | 132  | Adds new data to the next block                |
| BlockGen    | 133  | Generates the hash                             |
| BlockFound  | 134  | Saves the block                                |
| HashVal     | 135  | Validates the calculated hash                  |
| HashValAck  | 136  | Sends back if the hash is ok or not            |
