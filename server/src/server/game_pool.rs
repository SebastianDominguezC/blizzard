use super::connection_wrapper::ConnectionWrapper;

use std::sync::{Arc, Mutex};

pub struct GamePool {
    pub game_connections: Vec<Arc<Mutex<ConnectionWrapper>>>,
}

impl GamePool {
    pub fn new(max_games: i32, max_players: i32) -> GamePool {
        // Game wrapper vec
        let mut game_connections = vec![];

        // Loop max games
        for i in 0..max_games {
            // Create port id
            let port = 7000 + i;

            // Push game wrapper
            game_connections.push(ConnectionWrapper::new(i as usize, port, max_players));
        }

        // Return game pool
        GamePool { game_connections }
    }

    pub fn find_empty_game(&self) -> Option<i32> {
        for game_connection in &self.game_connections {
            let game_connection = game_connection.lock().unwrap();
            if game_connection.is_empty() {
                return Some(game_connection.port);
            }
        }
        None
    }
}
