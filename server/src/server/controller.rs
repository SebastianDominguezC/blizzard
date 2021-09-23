use crate::game::Player;
use crate::server::connector::Connector;
use crate::server::signal::Signal;

use blizzard_engine::core::application::{Application, Message};
use blizzard_engine::game::Game;

use serde::Serialize;
use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::time::Duration;
use uid::Uid;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct SharedState {
    counter: i32,
}

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

    pub fn open_game_port<T: Game<K>, K>(
        port: i32,
        max_players: i32,
        connector: Arc<Mutex<Connector>>,
        mut app: Application<T, K>,
    ) where
        T: Send + 'static,
        K: Send + Serialize + 'static,
    {
        // Store port id
        let id = port;

        // Create message channel
        let (tx, rx) = mpsc::channel();

        // Get shared state between controller and app
        let shared_state = Arc::clone(&app.shared_state);

        // Create controller mutex with reference counter
        let controller = Arc::new(Mutex::new(Controller::new(port, max_players, connector)));

        // Format port
        let port = format!("0.0.0.0:{}", port);
        println!("Opening game in port {}", port);

        // Start app / game in a new thread
        let builder = thread::Builder::new().name(format!("App-thread-{}", id));
        builder
            .spawn(move || {
                app.start(rx);
            })
            .expect("Could not create thread");

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

                        // Create concurrency clones
                        let sender = tx.clone();
                        let shared_state = Arc::clone(&shared_state);

                        // Spawn thread and move thread and controller
                        let builder = thread::Builder::new()
                            .name(format!("Game-{}-player-{}", id, player_id));
                        builder
                            .spawn(move || {
                                Controller::handle_player_connection::<K>(
                                    stream,
                                    controller,
                                    player_id,
                                    sender,
                                    shared_state,
                                )
                                .unwrap_or_else(|error| eprintln!("{:?}", error));
                            })
                            .expect("Could not create thread");
                    } else {
                        stream.write("Could not join".as_bytes()).unwrap();
                    }
                }
            }
        }
    }

    pub fn handle_player_connection<K>(
        stream: TcpStream,
        game: Arc<Mutex<Controller>>,
        id: usize,
        sender: Sender<Message>,
        shared_state: Arc<Mutex<K>>,
    ) -> Result<(), Error>
    where
        K: Send + Serialize + 'static,
    {
        println!("Connecting player {} to game", id);

        let mut stream_clone = stream.try_clone().unwrap();

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
                let player_index = game.players.iter().position(|p| p.id == id).unwrap();

                // If no bytes end connection
                if bytes_read == 0 {
                    game.connector.lock().unwrap().remove_player();
                    game.remove_player(player_index);
                    break;
                }

                let json = str::from_utf8(&buffer).unwrap();
                let signal: Signal = serde_json::from_str(&json.trim()).unwrap();

                match signal {
                    Signal::MovePlayer(pos) => {
                        let sent = sender.send(Message::A);
                        match sent {
                            Ok(y) => {}
                            Err(e) => {}
                        }
                    }
                    _ => {
                        println!("Some other game logic");
                    }
                }
            }
        });

        // Stream sender
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(1000));

                // On stream input, aquire shared state lock
                let reduced_state = shared_state.lock().unwrap();

                let serialized = serde_json::to_string(&*reduced_state);

                if let Ok(s) = serialized {
                    // Write back to stream
                    let s = s + "\n";
                    stream_clone.write(s.as_bytes());
                }
            }
        });

        return Ok(());
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
