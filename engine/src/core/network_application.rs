//! # Network Application
//!
//! This application runs the game, with messaging capabilities that come from the server.

use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::core::logger::initialize_logging;
use crate::game::Game;

/// Application that runs the app.
/// App will handle all engine features.
/// Missing:
/// * Renderer
/// * Window abstraction
/// * Event handler
/// * Many, many more features...
///
/// # Type definitions
/// * T: Game type
/// * K: Shared state between client
/// * I: Input for handling client
/// * M: Message for sharing between app and server controller
pub struct Application<T: Game<K, I>, K, I> {
    pub is_running: bool,
    pub shared_state: Arc<Mutex<K>>,
    is_suspended: bool,
    last_time: Instant,
    frames_per_second: Duration,
    input: Arc<Mutex<I>>,
    game: T,
}

impl<T: Game<K, I>, K, I> Clone for Application<T, K, I>
where
    T: Clone,
    K: Copy,
    I: Copy,
{
    fn clone(&self) -> Application<T, K, I> {
        Application {
            is_running: self.is_running,
            shared_state: Arc::new(Mutex::new(*self.shared_state.lock().unwrap())),
            is_suspended: self.is_suspended,
            last_time: self.last_time,
            frames_per_second: self.frames_per_second,
            input: Arc::new(Mutex::new(*self.input.lock().unwrap())),
            game: self.game.clone(),
        }
    }
}

impl<T: Game<K, I>, K, I> Application<T, K, I> {
    /// Create application
    /// Starts logging
    fn create(game: T, shared_state: K, input: I, game_update_rate: i32) -> Application<T, K, I> {
        // Start logging
        initialize_logging();

        // Return app
        Application {
            is_running: false,
            is_suspended: false,
            last_time: Instant::now(),
            frames_per_second: Duration::from_millis((1000 / game_update_rate) as u64), // 1000 / millis = frames per sec
            input: Arc::new(Mutex::new(input)),
            shared_state: Arc::new(Mutex::new(shared_state)),
            game,
        }
    }

    /// Run the app
    /// Starts the game loop
    pub fn start<M>(
        &mut self,
        receiver: Receiver<M>,
        handle_input: &'static (dyn Fn(Receiver<M>, Arc<Mutex<I>>) -> I + Sync),
    ) where
        M: Send,
        I: Send + Copy,
    {
        self.is_running = true;

        // game configuration
        self.game.world_config();

        let input_copy = Arc::clone(&self.input);

        // Spawn a handler for client input
        thread::spawn(move || {
            handle_input(receiver, input_copy);
        });

        // game loop
        while self.is_running {
            // self.last_time += 1.0;
            let sleep_time = self.frames_per_second - self.last_time.elapsed();
            thread::sleep(sleep_time);

            // initial time
            self.last_time = Instant::now();

            // update
            self.game
                .update(*self.input.lock().unwrap(), Arc::clone(&self.shared_state));

            // Reset input
            self.game.reset_input(Arc::clone(&self.input));
            // render

            // End the game
            self.is_running = !self.game.end_game();
        }
    }
}

/// Create app and return it
pub fn create_app<T: Game<K, I>, K, I>(
    game: T,
    shared_state: K,
    input: I,
    game_update_rate: i32,
) -> Application<T, K, I> {
    Application::create(game, shared_state, input, game_update_rate)
}
