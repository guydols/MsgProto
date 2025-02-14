# MsgProto

Proof of Concept for implementing own RPC-like client server model.

## Description

MsgProto is a proof-of-concept project to implement a custom RPC-like client-server model using Rust. This repository aims to demonstrate the basic principles of remote procedure calls in a client-server architecture. Hopefully this show how simple the setup can be and how easily you can abstract away the lower level networking stuff.

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