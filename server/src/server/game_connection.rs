use crate::game::Player;
use crate::server::connection_wrapper::ConnectionWrapper;
use crate::server::signal::Signal;

use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::time::Duration;
use uid::Uid;

use std::sync::{Arc, Mutex, MutexGuard};

pub struct GameConnection {
    players: Vec<Player>,
    max_players: i32,
    id: i32,
    wrapper_ref: Arc<Mutex<ConnectionWrapper>>,
}

impl GameConnection {
    fn new(
        id: i32,
        max_players: i32,
        game_wrapper: Arc<Mutex<ConnectionWrapper>>,
    ) -> GameConnection {
        GameConnection {
            players: vec![],
            max_players,
            id,
            wrapper_ref: game_wrapper,
        }
    }

    pub fn connect_to_game(
        port: i32,
        max_players: i32,
        game_wrapper: Arc<Mutex<ConnectionWrapper>>,
    ) {
        // Create game mutex with reference counter
        let game = Arc::new(Mutex::new(GameConnection::new(
            port,
            max_players,
            game_wrapper,
        )));

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
                            GameConnection::connect_player(stream, game, player_id)
                                .unwrap_or_else(|error| eprintln!("{:?}", error));
                        });
                    } else {
                        stream.write("Could not join".as_bytes()).unwrap();
                    }
                }
            }
        }
    }

    pub fn connect_player(
        stream: TcpStream,
        game: Arc<Mutex<GameConnection>>,
        id: usize,
    ) -> Result<(), Error> {
        println!("Connecting player {} to game", id);

        let mut stream_clone = stream.try_clone().unwrap();
        let game_clone = Arc::clone(&game);

        // Stream receiver
        thread::spawn(move || {
            loop {
                let mut buffer: Vec<u8> = Vec::new();
                let mut reader = BufReader::new(&stream);
                reader
                    .read_until(b'\n', &mut buffer)
                    .expect("Could not read into buffer");
                let bytes_read = buffer.len();

                // On stream input, aquire game lock and move player
                let mut game = game.lock().unwrap();

                // Get data from game
                let index = game.players.iter().position(|p| p.id == id).unwrap();

                // If no bytes end connection
                if bytes_read == 0 {
                    game.wrapper_ref.lock().unwrap().remove_player();
                    game.remove_player(index);
                    break;
                }

                let json = str::from_utf8(&buffer).unwrap();
                let signal: Signal = serde_json::from_str(&json.trim()).unwrap();

                GameConnection::handle_signal(signal, game, index);
            }
        });

        // Stream sender
        thread::spawn(move || {
            let game = game_clone;
            loop {
                thread::sleep(Duration::from_millis(30));

                // On stream input, aquire game lock and move player
                let game = game.lock().unwrap();

                let serialized = serde_json::to_string(&game.players);

                if let Ok(s) = serialized {
                    // Write back to stream
                    let s = s + "\n";
                    stream_clone.write(s.as_bytes());
                }
            }
        });

        return Ok(());
    }

    pub fn handle_signal(
        signal: Signal,
        mut game: MutexGuard<GameConnection>,
        player_index: usize,
    ) {
        match signal {
            Signal::MovePlayer(pos) => {
                let player = &mut game.players[player_index];
                player.move_player(pos.x, pos.y);
            }
            _ => {
                println!("Some other game logic");
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
