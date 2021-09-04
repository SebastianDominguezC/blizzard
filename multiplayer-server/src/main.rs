#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use std::sync::{
    // mpsc::{self, Receiver, Sender},
    Arc,
    Mutex,
};

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

struct GameWrapper {
    player_count: usize,
    max_players: i32,
    game_started: bool,
    port: i32,
    id: usize,
}

impl GameWrapper {
    fn new(id: usize, port: i32, max_players: i32) -> Arc<Mutex<GameWrapper>> {
        // Create game wrapper
        let game_wrapper = GameWrapper {
            player_count: 0,
            game_started: false,
            id,
            port,
            max_players,
        };

        // Wrap in arc mutex
        let game_wrapper = Arc::new(Mutex::new(game_wrapper));

        // Clone game wrapper for game thread
        let wrapper_clone = Arc::clone(&game_wrapper);

        // Spawn Game Thread
        thread::spawn(move || {
            println!("Running game in port {}", port);

            // Run a game
            Game::run_game(port, max_players, wrapper_clone);
        });

        return game_wrapper;
    }

    fn is_empty(&self) -> bool {
        self.player_count < self.max_players as usize && !self.game_started
    }

    fn add_player(&mut self) {
        self.player_count += 1;
    }
}

struct Game {
    players: Vec<Player>,
    max_players: i32,
    id: i32,
    wrapper_ref: Arc<Mutex<GameWrapper>>,
}

impl Game {
    fn run_game(port: i32, max_players: i32, game_wrapper: Arc<Mutex<GameWrapper>>) {
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
                            handle_player(stream, game, player_id)
                                .unwrap_or_else(|error| eprintln!("{:?}", error));
                        });
                    } else {
                        stream.write("Could not join".as_bytes()).unwrap();
                    }
                }
            }
        }
    }

    fn new(id: i32, max_players: i32, game_wrapper: Arc<Mutex<GameWrapper>>) -> Game {
        Game {
            players: vec![],
            max_players,
            id,
            wrapper_ref: game_wrapper,
        }
    }

    fn add_player(&mut self) -> (bool, usize) {
        if self.players.len() < self.max_players as usize {
            let id = self.players.len();
            let new_player = Player::new(id);
            self.players.push(new_player);
            return (true, id);
        }
        (false, 0)
    }
}

// Handles a single player
fn handle_player(mut stream: TcpStream, game: Arc<Mutex<Game>>, id: usize) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        // Read client
        let bytes_read = stream.read(&mut buf)?;

        // If no bytes end connection
        if bytes_read == 0 {
            return Ok(());
        }

        // On stream input, aquire game lock and move player
        let mut game = game.lock().unwrap();
        let game_id = game.id;

        {
            let player = &mut game.players[id];
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    // #[serde(rename = "lolId")]
    id: usize,
    pos: Position,
}

impl Player {
    fn new(id: usize) -> Player {
        Player {
            id,
            pos: Position { x: 0, y: 0 },
        }
    }

    fn move_player(&mut self) {
        self.pos.displace_unit();
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn displace_unit(&mut self) {
        self.x += 1;
        self.y += 1;
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

    // game_pool.start_game_listeners();

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
