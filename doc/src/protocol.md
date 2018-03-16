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
- `Payload` - Payload is dependent on the `Type`

### Example Byte Array

`[1, 1, 40, 19, 197, 47, 0]`

- `1` -> Version
- `1` -> Type
- `40 19 197 47` -> Checksum
- `0` -> Payload

## Type

The Type field determines the payload.
Besides that also the application decides what to do with the incoming message.
In the next chapters an overview of all currenlty avaialable Types are listed and described.

### Ping

Simple Ping.
Used to check if a peer is still alive.
The Pinged peer should send back a pong.

No payload.

### Pong

Simple Pong.
Should be used to answer an Ping Type.

No payload.

### Punsh

### GetPeers

### GetPeersAck

### Register

### RegisterAck

### GetBlocks

### GetBlocksAck

### GetBlock

### GetBlockAck

### BlockData

### BlockGen

### BlockFound

### HashVal

### HashValAck

### NotAValidEvent
