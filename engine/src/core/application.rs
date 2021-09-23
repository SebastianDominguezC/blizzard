use crate::core::logger::initialize_logging;
use crate::game::Game;

use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// TODO: Custom messages + handles !!! -> For client message handling
pub enum Message {
    A,
}

pub struct ReducedGameState {}

pub struct Application<T: Game<K>, K> {
    pub is_running: bool,
    pub shared_state: Arc<Mutex<K>>,
    is_suspended: bool,
    last_time: Instant,
    frames_per_second: Duration,
    input: Arc<Mutex<i32>>,
    game: T,
}

impl<T: Game<K>, K> Clone for Application<T, K>
where
    T: Clone,
    K: Copy,
{
    fn clone(&self) -> Application<T, K> {
        Application {
            is_running: self.is_running,
            shared_state: Arc::new(Mutex::new(*self.shared_state.lock().unwrap())),
            is_suspended: self.is_suspended,
            last_time: self.last_time,
            frames_per_second: self.frames_per_second,
            input: Arc::new(Mutex::new(0)),
            game: self.game.clone(),
        }
    }
}

impl<T: Game<K>, K> Application<T, K> {
    pub fn create(game: T, shared_state: K) -> Application<T, K> {
        // Start logging
        initialize_logging();

        // Return app
        Application {
            is_running: false,
            is_suspended: false,
            last_time: Instant::now(),
            frames_per_second: Duration::from_millis(1000), // 1000 / millis = frames per sec
            input: Arc::new(Mutex::new(0)),
            shared_state: Arc::new(Mutex::new(shared_state)),
            game,
        }
    }
    pub fn start(&mut self, receiver: Receiver<Message>) {
        self.is_running = true;

        // game configuration
        self.game.world_config();

        let input_copy = Arc::clone(&self.input);

        // TODO: Custom messages + handles !!! -> For client message handling
        thread::spawn(move || {
            let input = input_copy;
            for m in receiver {
                match m {
                    Message::A => {
                        *input.lock().unwrap() += 1;
                    }
                }
            }
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

            // render

            // End the game
            self.is_running = !self.game.end_game();

            // *self.shared_state.lock().unwrap() += *self.input.lock().unwrap();
        }
    }
}

pub fn create_app<T: Game<K>, K>(game: T, shared_state: K) -> Application<T, K> {
    Application::create(game, shared_state)
}
