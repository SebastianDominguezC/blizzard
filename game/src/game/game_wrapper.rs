use super::Game;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct GameWrapper {
    player_count: usize,
    max_players: i32,
    game_started: bool,
    id: usize,
    pub port: i32,
}

impl GameWrapper {
    pub fn new(id: usize, port: i32, max_players: i32) -> Arc<Mutex<GameWrapper>> {
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

    pub fn is_empty(&self) -> bool {
        self.player_count < self.max_players as usize && !self.game_started
    }

    pub fn add_player(&mut self) {
        self.player_count += 1;
    }

    pub fn remove_player(&mut self) {
        self.player_count -= 1;
    }
}
