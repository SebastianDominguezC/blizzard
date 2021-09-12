use crate::game::GameWrapper;

use std::sync::{Arc, Mutex};

pub struct GamePool {
    pub game_wrappers: Vec<Arc<Mutex<GameWrapper>>>,
}

impl GamePool {
    pub fn new(max_games: i32, max_players: i32) -> GamePool {
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

    pub fn find_empty_game(&self) -> Option<i32> {
        for game_wrapper in &self.game_wrappers {
            let game_wrapper = game_wrapper.lock().unwrap();
            if game_wrapper.is_empty() {
                return Some(game_wrapper.port);
            }
        }
        None
    }
}
