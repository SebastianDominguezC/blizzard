mod game_wrapper;
mod player;
mod position;

pub use game_wrapper::GameWrapper;
pub use player::Player;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use uid::Uid;

use std::sync::{Arc, Mutex};

pub struct Game {
    players: Vec<Player>,
    max_players: i32,
    id: i32,
    wrapper_ref: Arc<Mutex<GameWrapper>>,
}

impl Game {
    fn new(id: i32, max_players: i32, game_wrapper: Arc<Mutex<GameWrapper>>) -> Game {
        Game {
            players: vec![],
            max_players,
            id,
            wrapper_ref: game_wrapper,
        }
    }

    pub fn handle_player(
        mut stream: TcpStream,
        game: Arc<Mutex<Game>>,
        id: usize,
    ) -> Result<(), Error> {
        println!("Incoming connection from: {}", stream.peer_addr()?);
        let mut buf = [0; 512];

        loop {
            // Read client
            let bytes_read = stream.read(&mut buf)?;

            // On stream input, aquire game lock and move player
            let mut game = game.lock().unwrap();
            let index = game.players.iter().position(|p| p.id == id).unwrap();

            // If no bytes end connection
            if bytes_read == 0 {
                game.wrapper_ref.lock().unwrap().remove_player();
                game.remove_player(index);
                return Ok(());
            }

            let game_id = game.id;
            {
                let player = &mut game.players[index];
                player.move_player();
            }

            let serialized = serde_json::to_string(&game.players);
            // Print new position
            println!("Server: {} is updating player {}", game_id, id);
            if let Ok(s) = serialized {
                // Write back to stream
                let s = s + "\n";
                stream.write(s.as_bytes())?;
            } else {
                stream.write(&buf[0..bytes_read])?;
            }
        }
    }

    pub fn run_game(port: i32, max_players: i32, game_wrapper: Arc<Mutex<GameWrapper>>) {
        // Create game mutex with reference counter
        let game = Arc::new(Mutex::new(Game::new(port, max_players, game_wrapper)));

        // Format port
        let port = format!("0.0.0.0:{}", port);

        // Create tcp listener
        let listener = TcpListener::bind(port).expect("Could not bind");

        for stream in listener.incoming() {
            match stream {
                Err(e) => {
                    eprintln!("failed: {}", e)
                }
                Ok(mut stream) => {
                    // Push a new player to the game
                    let (could_join, player_id) = game.lock().unwrap().add_player();

                    if could_join {
                        // Clone the game
                        let game = Arc::clone(&game);

                        // Update game wrapper player count
                        game.lock()
                            .unwrap()
                            .wrapper_ref
                            .lock()
                            .unwrap()
                            .add_player();

                        // Spawn thread and move thread and game
                        thread::spawn(move || {
                            Game::handle_player(stream, game, player_id)
                                .unwrap_or_else(|error| eprintln!("{:?}", error));
                        });
                    } else {
                        stream.write("Could not join".as_bytes()).unwrap();
                    }
                }
            }
        }
    }

    pub fn add_player(&mut self) -> (bool, usize) {
        if self.players.len() < self.max_players as usize {
            let id = Uid::new_numerical(4) as usize;
            let new_player = Player::new(id);
            self.players.push(new_player);
            return (true, id);
        }
        (false, 0)
    }

    pub fn remove_player(&mut self, index: usize) -> bool {
        if self.players.len() == 0 {
            return false;
        }
        self.players.remove(index);
        true
    }
}
