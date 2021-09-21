use crate::server::controller::Controller;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Connector {
    pub port: i32,
    max_players: i32,
    player_count: usize,
}

impl Connector {
    pub fn new(port: i32, max_players: i32) -> Arc<Mutex<Connector>> {
        // Create game wrapper
        let game_connector = Connector {
            player_count: 0,
            port,
            max_players,
        };

        let game_connector = Arc::new(Mutex::new(game_connector));

        // Clone game connector for game controller
        let connector_clone = Arc::clone(&game_connector);

        // Spawn game controller thread
        thread::spawn(move || {
            // Start a controller
            Controller::open_game_port(port, max_players, connector_clone);
        });

        return game_connector;
    }

    pub fn is_empty(&self) -> bool {
        self.player_count < self.max_players as usize
    }

    pub fn add_player(&mut self) {
        self.player_count += 1;
    }

    pub fn remove_player(&mut self) {
        self.player_count -= 1;
    }
}
