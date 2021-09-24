# ❄️ Blizzard ❄️

This is the official Blizzard Game Engine and Server Engine repository!

This repository is a workspace of all the Blizzard libraries, to develop and publish them in an efficient way.
This is not a full fledged proyect. As of now, the game engine is just an ECS data/game engine.
Only basic data-driven games without rendering and extreme features can be made with this engine.
The server uses this data engine in order to provide an authorative client-server architecture.
The code is open for anyone who wants to tweak the proyect and improve it for their own needs,
however this is a personal proyect and no contributions will be accepted (this will likely change in the future).

## Origin

This proyect started as my curiosity to develop a multiplayer game. I ended up deciding to write my own game server using Rust.
After some succesfull connections and data passing between threads, I realised that the server needed a game engine in order to implement
an authorative client-server achitecture. This gave birth to the Blizzard Game Engine and Server Engine! If you are curious
about my learning process and development process, please enter the `development process` section in the website.

## Example

A simple example of using the game engine with the server is in the `example` folder.
It is a library with two binaries. To try the example run the following in your terminal:

Start server:

```
cd example
cargo build
cargo run --bin server
```

In a new terminal, start a client:

```
cd example
cargo run --bin client
```

The server opens up 4 games, each with a unique TCP port. The server handles client connections/disconnections.

When running the client, you have to enter a username (which is not used, oops).
After that, the terminal will print the shared state definied by the server.
On the terminal, you can update your player's position by entering any of the following:

```
w
a
s
d
close
```

`Close` is for disconnecting.

If you run another client, it will join the game of the first client!
If you run a 3rd client, it will join another game port, since the server is definied with a max player capacity of 2.
You can continue to run clients, until the server is full!

Please read over the code to fully understand what is going on!

## Blizzard overview

### ❄️ Blizzard Server Engine ❄️

The server has the objective to be an authorative server that supports TCP and UDP multiplayer games.
As of now it only supports TCP games.
The server is inside the `server` folder. The server provides the server struct, where one can start a server:

```
Server::new(...)
```

The proyect is developed with generics, allowing full flexibility for users to develop their own games with many data structures.
The example highlights a basic implementation.
See the website section `learn` to better understand how to develop your own multiplayer games!

#### Roadmap

Some features that are considered for the future:

- UDP application support
- Cheating prevention
- Better error handling
- Performance improvements / diagnostics
- AI/ML enhancements
- Extensive logging

### ❄️ Blizzard Game Engine ❄️

The game engine is meant to be a stand-alone ECS game engine that can be used modularily, meaning
a user can only use the parts that are required. As of now, it only supports a network application
that uses ECS architecture for it's data, as the original proyect was focused on running a multiplayer game.
The game engine is inside the `engine` folder.

As of now, it is only a data/game engine.

See the website section `learn` to better understand how to develop your own games (not necessarily multiplayer)!

#### Roadmap

Some features that are considered for the future:

- Multiplatform-support (Windows and iOS)
- Window abstractions
- Renderer API
- Event handler
- Better logging
- AI/ML functionalities
- Performance improvements
- Ready ECS Components like physics
- Math and Physics libraries
- Sound support
