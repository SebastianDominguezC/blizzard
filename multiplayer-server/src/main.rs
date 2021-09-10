extern crate game;

use game::game::GameWrapper;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

struct GamePool {
    game_wrappers: Vec<Arc<Mutex<GameWrapper>>>,
}

impl GamePool {
    fn new(max_games: i32, max_players: i32) -> GamePool {
        // Game wrapper vec
        let mut game_wrappers = vec![];

        // Loop max games
        for i in 0..max_games {
            // Create port id
            let port = 7000 + i;

            // Push game wrapper
            game_wrappers.push(GameWrapper::new(i as usize, port, max_players));
        }

        // Return game pool
        GamePool { game_wrappers }
    }

    fn find_empty_game(&self) -> Option<i32> {
        for game_wrapper in &self.game_wrappers {
            let game_wrapper = game_wrapper.lock().unwrap();
            if game_wrapper.is_empty() {
                return Some(game_wrapper.port);
            }
        }
        None
    }
}

// Handles a single client
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

fn main() {
    // Create game pool
    let game_pool = GamePool::new(24, 2);

    // Create TCP listener
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("failed: {}", e)
            }
            Ok(stream) => {
                // Handle connection and find a game
                if let Some(game_wrapper_port) = game_pool.find_empty_game() {
                    handle_client_connection(stream, game_wrapper_port)
                        .unwrap_or_else(|error| eprintln!("{:?}", error));
                } else {
                    handle_client_connection(stream, 0)
                        .unwrap_or_else(|error| eprintln!("{:?}", error));
                }
            }
        }
    }
}
