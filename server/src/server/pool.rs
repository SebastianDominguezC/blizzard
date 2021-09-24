//! # Pool
//! The pool is in charge of finding emtpy games to connect the client.

use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use serde::de::DeserializeOwned;
use serde::Serialize;

use blizzard_engine::core::network_application::create_app;
use blizzard_engine::game::Game;

use crate::server::connector::Connector;

/// A pool of game connectors
/// Pool finds empty games and returns to the client empty game port.
/// # Example
/// For a working example, please see official github repository, in the example lib.
///
pub struct Pool {
    game_connectors: Vec<Arc<Mutex<Connector>>>,
}

impl Pool {
    /// Creates a new game.
    /// # Type definitions:
    /// * T: Game type
    /// * K: Shared state type (to share state to client)
    /// * I: Input type (to manipulate user input inside app)
    /// * M: Message type (sent from server to app)
    pub fn new<T: Game<K, I>, K, I, M>(
        max_games: i32,
        max_players: i32,
        game: T,
        shared_state: K,
        input: I,
        handle_input: &'static (dyn Fn(Receiver<(M, usize)>, Arc<Mutex<I>>) -> I + Sync),
        send_data_rate: i32,
        game_update_rate: i32,
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
            let app = create_app(game.clone(), shared_state.clone(), input, game_update_rate);

            // Push new game connector
            game_connectors.push(Connector::new(
                port,
                max_players,
                app,
                handle_input,
                send_data_rate,
            ));
        }

        // Return game pool
        Pool { game_connectors }
    }

    /// Finds an empty game and returns the port if there is an empty game
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
