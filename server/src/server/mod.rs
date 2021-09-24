//! # Server
//! The server is in charge of creating the game pool.
//!
//! and giving a client connection the port of an emtpy game that it can connect to.

mod connector;
pub mod controller;
mod pool;

use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use serde::de::DeserializeOwned;
use serde::Serialize;

use blizzard_engine::game::Game;

use pool::Pool;

/// Server struct for creating servers.
/// # Example
/// For a working example, please see official github repository, in the example lib.
///
pub struct Server {}

impl Server {
    /// Start a new server
    pub fn new<T: Game<K, I>, K, I, M>(
        port: i32,
        max_games: i32,
        max_players: i32,
        game: T,
        shared_state: K,
        input: I,
        handle_input: &'static (dyn Fn(Receiver<(M, usize)>, Arc<Mutex<I>>) -> I + Sync),
        send_data_rate: i32,
        game_update_rate: i32,
    ) where
        T: Clone + Send + 'static,
        K: Clone + Send + Serialize + 'static,
        I: Send + Copy,
        M: Send + DeserializeOwned,
    {
        // Create game pool
        let game_pool = Pool::new(
            max_games,
            max_players,
            game,
            shared_state,
            input,
            handle_input,
            send_data_rate,
            game_update_rate,
        );

        // Format TCP to local machine
        let tcp = format!("0.0.0.0:{}", port);

        // Create TCP listener
        let listener = TcpListener::bind(tcp).expect("Could not bind");

        // For every new connection
        for stream in listener.incoming() {
            match stream {
                Err(e) => {
                    eprintln!("failed: {}", e)
                }
                Ok(stream) => {
                    // Handle connection and find a game
                    if let Some(game_wrapper_port) = game_pool.find_empty_game() {
                        Server::handle_client_connection(stream, game_wrapper_port)
                            .unwrap_or_else(|error| eprintln!("{:?}", error));
                    } else {
                        Server::handle_client_connection(stream, 0)
                            .unwrap_or_else(|error| eprintln!("{:?}", error));
                    }
                }
            }
        }
    }

    /// Handles client connections if a game was found.
    ///
    /// Sends the client the port of the game.
    ///
    /// Client should then connect to the new port.
    fn handle_client_connection(mut stream: TcpStream, port: i32) -> Result<(), Error> {
        println!("Incoming connection from: {}", stream.peer_addr()?);
        let mut buf = [0; 512];
        loop {
            let bytes_read = stream.read(&mut buf)?;
            if bytes_read == 0 {
                return Ok(());
            }
            let port: String = format!("{}\n", port);
            stream.write(port.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
