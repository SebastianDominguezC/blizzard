use crate::server::connector::Connector;

use std::sync::{Arc, Mutex};

pub struct Pool {
    game_connectors: Vec<Arc<Mutex<Connector>>>,
}

impl Pool {
    pub fn new(max_games: i32, max_players: i32) -> Pool {
        // Game wrapper vec
        let mut game_connectors = vec![];

        // Loop max games
        for i in 0..max_games {
            // Create port id
            let port = 7000 + i;

            // Push game connector
            game_connectors.push(Connector::new(port, max_players));
        }

        // Return game pool
        Pool { game_connectors }
    }

    pub fn find_empty_game(&self) -> Option<i32> {
        for game_connector in &self.game_connectors {
            let game_connector = game_connector.lock().unwrap();
            if game_connector.is_empty() {
                return Some(game_connector.port);
            }
        }
        None
    }
}
