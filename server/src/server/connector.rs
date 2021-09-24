use blizzard_engine::core::network_application::Application;
use blizzard_engine::game::Game;

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::server::controller::Controller;

pub struct Connector {
    pub port: i32,
    max_players: i32,
    player_count: usize,
}

impl Connector {
    pub fn new<'de, T: Game<K, I>, K, I, M>(
        port: i32,
        max_players: i32,
        app: Application<T, K, I>,
        handle_input: &'static (dyn Fn(Receiver<M>, Arc<Mutex<I>>) -> I + Sync),
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
        let game_connector = Arc::new(Mutex::new(game_connector));

        // Clone game connector for game controller
        let connector_clone = Arc::clone(&game_connector);

        // Spawn thread for handling connections
        let builder = thread::Builder::new().name(format!("Controller-{}", port));

        builder
            .spawn(move || {
                Controller::open_game_port(port, max_players, connector_clone, handle_input, app);
            })
            .expect("Could not create thread");

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
