//! # Connector
//! The connector passes information between the game pool and controller.
//! It is in charge of enabling "connecting" capabilities.

use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

use serde::de::DeserializeOwned;
use serde::Serialize;

use blizzard_engine::core::network_application::Application;
use blizzard_engine::game::Game;

use crate::server::controller::Controller;

/// # Functionality
/// * Connects clients to game
/// * Provides information to pool
/// * Tracks player counts
/// # Example
/// For a working example, please see official github repository, in the example lib
pub struct Connector {
    pub port: i32,
    max_players: i32,
    player_count: usize,
}

impl Connector {
    /// Creates a game connector
    pub fn new<T: Game<K, I>, K, I, M>(
        port: i32,
        max_players: i32,
        app: Application<T, K, I>,
        handle_input: &'static (dyn Fn(Receiver<(M, usize)>, Arc<Mutex<I>>) -> I + Sync),
        send_data_rate: i32,
    ) -> Arc<Mutex<Connector>>
    where
        T: Send + 'static,
        K: Send + Serialize + 'static,
        I: Send + Copy,
        M: Send + DeserializeOwned,
    {
        // Create game wrapper
        let game_connector = Connector {
            player_count: 0,
            port,
            max_players,
        };

        // Clone game connector for game controller
        let game_connector = Arc::new(Mutex::new(game_connector));
        let connector_clone = Arc::clone(&game_connector);

        // Spawn thread for handling connections
        let builder = thread::Builder::new().name(format!("Controller-{}", port));

        builder
            .spawn(move || {
                Controller::open_game_port(
                    port,
                    max_players,
                    connector_clone,
                    handle_input,
                    app,
                    send_data_rate,
                );
            })
            .expect("Could not create thread");

        // Return connector for pool
        return game_connector;
    }

    /// Determine if game is no full, to add new players
    pub fn is_empty(&self) -> bool {
        self.player_count < self.max_players as usize
    }

    /// Add a player
    pub fn add_player(&mut self) {
        self.player_count += 1;
    }

    /// Remove a player
    pub fn remove_player(&mut self) {
        self.player_count -= 1;
    }
}
