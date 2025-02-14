# MsgProto

Proof of Concept for implementing own RPC-like client server model.

## Description

MsgProto is a proof-of-concept project to implement a custom RPC-like client-server model using Rust. This repository aims to demonstrate the basic principles of remote procedure calls in a client-server architecture. Hopefully this show how simple the setup can be and how easily you can abstract away the lower level networking stuff.

#### TCP
```tcp.rs``` contains 2 helper functions to send en receive framed TCP data.
Framed meaning that the first usized sized data in the TCP data tells you the amount of bytes to except.
This gives us the ability to send serialized data over TCP and deserialize on the other side.

#### Message
```message.rs``` contains the message structure and functionality to handle received messages or create new ones.
Using ```#[derive(Encode, Decode)]``` we can (de)serialize the structures to send them over the network.
These definitions can be part of the shared library so that client and server have exactly the same definitions.

#### Client Server
The ```client_main.rs``` and ```server_main.rs``` contain some basic code to show they can communicate back and forth using HeartBeats.
In a proper implementation you could save the information of connecting clients on the server, this will also ensure the connection is not destroyed.
You could also implement some kind of state table on client and/or server to be able to track (if an reply is not directly available) what to still reply.

## Installation

Clone the repository:
```bash
git clone https://github.com/guydols/MsgProto.git
cd MsgProto
```

Usage
Run the server and in a seperate terminal run the client:
```bash
cargo run --bin server

cargo run --bin client
```


For any questions or comments, feel free to contact me.
