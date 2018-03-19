# Misc Types

## Ping

Simple Ping.
Used to check if a peer is still alive.
The Pinged peer should send back a pong.

No payload.

## Pong

Simple Pong.
Should be used to answer a Ping Type.

No payload.

## Punsh

Send by the hole puncher to notify another peer in an other network.
The receving peers will then try to connect with the other peer.
During that process the port to the other peer gets an entry in the NAT.
Becuase both do this, the peers should be able to connect to each other.
Even if they are in seperate networks.

### Payload

Payload is only a single string containg the IP-Adress of the other peer.

## NotAValidType

Represents an unknown type.
This should never be used.