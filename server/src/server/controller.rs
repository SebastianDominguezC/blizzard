//! # Controller
//! The controller is in charge of opening the game ports and handling client connections to games.
//! It opens threads per client.
//! Each client has a receiver and a sender thread.

use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;

use blizzard_engine::core::network_application::Application;
use blizzard_engine::game::Game;
use blizzard_id::Uid;

use crate::game::Player;
use crate::server::connector::Connector;

/// # Functionality:
/// * Connection controller
/// * Provides information to connector
/// * Handles client connections to the game
pub struct Controller {
    // Waiting for future use
    port: i32,
    connector: Arc<Mutex<Connector>>,
    players: Vec<Player>,
    max_players: i32,
}

impl Controller {
    /// Create a new controller from a connector
    fn new(port: i32, max_players: i32, connector: Arc<Mutex<Connector>>) -> Controller {
        Controller {
            players: vec![],
            max_players,
            port,
            connector,
        }
    }

    /// What it does:
    /// * Runs a game (application).
    /// * Creates messaging channel between controller and app.
    /// * Creates a shared state to share between app and client.
    /// * Opens a port for game.
    pub fn open_game_port<'de, T: Game<K, I>, K, I, M>(
        port: i32,
        max_players: i32,
        connector: Arc<Mutex<Connector>>,
        handle_input: &'static (dyn Fn(Receiver<(M, usize)>, Arc<Mutex<I>>) -> I + Sync),
        mut app: Application<T, K, I>,
        send_data_rate: i32,
    ) where
        T: Send + 'static,
        K: Send + Serialize + 'static,
        I: Send + Copy,
        M: Send + DeserializeOwned,
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
                app.start(rx, handle_input);
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
                                Controller::handle_player_connection::<K, M>(
                                    stream,
                                    controller,
                                    player_id,
                                    sender,
                                    shared_state,
                                    send_data_rate,
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

    /// Handles player writing and reading
    pub fn handle_player_connection<'de, K, M>(
        stream: TcpStream,
        game: Arc<Mutex<Controller>>,
        id: usize,
        sender: Sender<(M, usize)>,
        shared_state: Arc<Mutex<K>>,
        send_data_rate: i32,
    ) -> Result<(), Error>
    where
        K: Send + Serialize + 'static,
        M: Send + DeserializeOwned + 'static,
    {
        println!("Connecting player {} to game", id);
        let mut stream_clone = stream.try_clone().unwrap();
        let sender = sender.clone();

        // Defines bool for dropping the thread on disconnection
        let drop_thread = Arc::new(Mutex::new(false));
        let drop_copy = Arc::clone(&drop_thread);

        // Stream receiver: Read from client
        thread::spawn(move || {
            let drop = drop_copy;
            loop {
                let mut buffer: Vec<u8> = Vec::new();
                let mut reader = BufReader::new(&stream);
                reader
                    .read_until(b'\n', &mut buffer)
                    .expect("Could not read into buffer");
                let bytes_read = buffer.len();

                // On stream input, get lock and aquire player id
                let mut game = game.lock().unwrap();
                let player_index = game.players.iter().position(|p| p.id == id).unwrap();

                // If no bytes end connection
                if bytes_read == 0 {
                    // Remove player
                    game.connector.lock().unwrap().remove_player();
                    game.remove_player(player_index);

                    // Mark thread for dropping
                    *drop.lock().unwrap() = true;
                    break;
                }

                // Parse message and send to app
                let json = str::from_utf8(&buffer).unwrap();
                let signal: M = serde_json::from_str(&json.trim()).unwrap();

                match sender.send((signal, id)) {
                    Ok(_) => {}
                    Err(_) => println!("Could not send signal to app."),
                }
            }
        });

        // Stream sender: write to client
        thread::spawn(move || {
            // 1000 / millis = frames per sec
            // millis = 1000 / frames_per_sec
            let sleep_time: u64 = (1000 / send_data_rate) as u64;

            // Client event loop
            loop {
                thread::sleep(Duration::from_millis(sleep_time));

                // On stream input, aquire shared state lock
                let reduced_state = shared_state.lock().unwrap();
                let serialized = serde_json::to_string(&*reduced_state);

                // Send state to client
                if let Ok(s) = serialized {
                    let s = s + "\n";
                    match stream_clone.write(s.as_bytes()) {
                        Ok(_) => {}
                        Err(_) => println!("Could not send data to client."),
                    }
                }

                // Drop thread when client disconnects
                if *drop_thread.lock().unwrap() {
                    break;
                }
            }
        });

        return Ok(());
    }

    /// Add a player to the game
    pub fn add_player(&mut self) -> (bool, usize) {
        if self.players.len() < self.max_players as usize {
            let id = Uid::new_numerical(4) as usize;
            let new_player = Player::new(id);
            self.players.push(new_player);
            return (true, id);
        }
        (false, 0)
    }

    /// Remove a player from the game
    pub fn remove_player(&mut self, index: usize) -> bool {
        if self.players.len() == 0 {
            return false;
        }
        self.players.remove(index);
        true
    }
}
