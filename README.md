# TCP-Multiplayer-Server

A tcp multiplayer server + client Rust workspace

## Summary

This is a workspace for developing a TCP server for a multiplayer game.

There are two main folders, the server and client

This repo is in development, it will support abstractions for later reuse and customization!

There will also be a section on how to Dockerize the server, and upload it to AWS EC2

Feel free to open issues to suggest improvements!

## TCP Server

The TCP server for the multiplayer game

This is a TCP server for a multiplayer game.

The server runs 24 games in different threads that listen to different ports.

Each game can contain up to 2 players.

When a client connects to the main server, the Game Pool looks for an available game.

It then returns the port number of that address.

The client (more on this later) will then connect to the game on its corresponding port.

The client side app simply sends automatic messages, where the server simply updates the user position a simple movement.

** The server can be configured for however many games needed with as many players per game as needed **

#### To run server

```
cargo run -p "multiplayer-server
```

You can also build the workspace first, then run the package:

```
cargo build
cargo run -p "multiplayer-client
```

## TCP client

This client connects to the running TCP game server

A message must be written to the server (a simple std input) to receive a port to join an available game

Once joined, automatic messages are sent where the server updates the player position

All position changes are logged on the tcp multiplayer server (here no changes are reflected)

#### To run client

Make sure to be running the multiplayer-server first!

```
cargo run -p "multiplayer-client
```

You can also build the workspace first, then run the package:

```
cargo build
cargo run -p "multiplayer-client
```
