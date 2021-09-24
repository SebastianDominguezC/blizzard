use crate::server::connector::Connector;
use blizzard_engine::core::network_application::create_app;
use blizzard_engine::game::Game;

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

pub struct Pool {
    game_connectors: Vec<Arc<Mutex<Connector>>>,
}

impl Pool {
    pub fn new<'de, T: Game<K, I>, K, I, M>(
        max_games: i32,
        max_players: i32,
        game: T,
        shared_state: K,
        input: I,
        handle_input: &'static (dyn Fn(Receiver<M>, Arc<Mutex<I>>) -> I + Sync),
    ) -> Pool
    where
        T: Clone + Send + 'static,
        K: Clone + Send + Serialize + 'static,
        I: Send + Copy,
        M: Send + DeserializeOwned,
    {
        // Game wrapper vec
        let mut game_connectors = vec![];

        // Loop max games
        for i in 0..max_games {
            // Create port id
            let port = 7000 + i;

            // Create a new app for each port specified
            let app = create_app(game.clone(), shared_state.clone(), input);

            // Push game connector
            game_connectors.push(Connector::new(port, max_players, app, handle_input));
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
