# Proof of concept
### Hole puncher

- Starting hole puncher!
- Hole puncher: "REGISTER | 0.0.0.0:39723"
- Hole puncher response "ACK_REGISTER | NO_PEER"
- Hole puncher: "REGISTER | 0.0.0.0:51001"
- Hole puncher response "ACK_REGISTER | 87.188.56.50:39723"

### Peer 1 :39723

- "37.120.161.134:45000"
- Got messag: "ACK_REGISTER | NO_PEER"
- Sending ping to new peer: "87.188.56.50:51001"
- PING from "87.188.56.50:51001"
- Send PONG

### Peer 2 :51001

- "37.120.161.134:45000"
- Got messag: "ACK_REGISTER | 87.188.56.50:39723"
- Sending PING to Peer: 87.188.56.50:39723
- PONG from "87.188.56.50:39723"