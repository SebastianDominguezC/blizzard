pub mod signal;

mod connector;
pub mod controller;
mod pool;

use blizzard_engine::game::Game;
use serde::Serialize;

use pool::Pool;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Server {}

impl Server {
    pub fn new<T: Game<K>, K>(port: i32, max_games: i32, max_players: i32, game: T, shared_state: K)
    where
        T: Clone + Send + 'static,
        K: Copy + Send + Serialize + 'static,
    {
        // Create game pool
        let game_pool = Pool::new(max_games, max_players, game, shared_state);

        let tcp = format!("0.0.0.0:{}", port);

        // Create TCP listener
        let listener = TcpListener::bind(tcp).expect("Could not bind");

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
