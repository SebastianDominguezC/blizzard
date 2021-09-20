use crate::game::Player;
use crate::server::connector::Connector;
use crate::server::signal::Signal;

use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::time::Duration;
use uid::Uid;

use std::sync::{Arc, Mutex, MutexGuard};

pub struct Controller {
    port: i32,
    connector: Arc<Mutex<Connector>>,
    players: Vec<Player>,
    max_players: i32,
}

impl Controller {
    fn new(port: i32, max_players: i32, connector: Arc<Mutex<Connector>>) -> Controller {
        Controller {
            players: vec![],
            max_players,
            port,
            connector,
        }
    }

    pub fn open_game_port(port: i32, max_players: i32, connector: Arc<Mutex<Connector>>) {
        // Create controller mutex with reference counter
        let controller = Arc::new(Mutex::new(Controller::new(port, max_players, connector)));

        // Format port
        let port = format!("0.0.0.0:{}", port);

        println!("Opening game in port {}", port);

        // Create tcp listener
        let listener = TcpListener::bind(port).expect("Could not bind");

        for stream in listener.incoming() {
            match stream {
                Err(e) => {
                    eprintln!("failed: {}", e)
                }
                Ok(mut stream) => {
                    // Push a new player to the game
                    let (could_join, player_id) = controller.lock().unwrap().add_player();

                    if could_join {
                        // Clone the controller
                        let controller = Arc::clone(&controller);

                        // Update game wrapper player count
                        controller
                            .lock()
                            .unwrap()
                            .connector
                            .lock()
                            .unwrap()
                            .add_player();

                        // Spawn thread and move thread and controller
                        thread::spawn(move || {
                            Controller::connect_player(stream, controller, player_id)
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
        game: Arc<Mutex<Controller>>,
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
                    game.connector.lock().unwrap().remove_player();
                    game.remove_player(index);
                    break;
                }

                let json = str::from_utf8(&buffer).unwrap();
                let signal: Signal = serde_json::from_str(&json.trim()).unwrap();

                Controller::handle_signal(signal, game, index);
            }
        });

        // Stream sender
        thread::spawn(move || {
            let game = game_clone;
            loop {
                thread::sleep(Duration::from_millis(1000));

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

    pub fn handle_signal(signal: Signal, mut game: MutexGuard<Controller>, player_index: usize) {
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
