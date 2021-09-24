// TODO: NOT NETWORKED APP
// use crate::core::logger::initialize_logging;
// use crate::game::Game;

// use uid::Uid;

// use std::thread;
// use std::time::{Duration, Instant};

// pub struct Application<T: Game<K, I>, K, I> {
//     pub is_running: bool,
//     is_suspended: bool,
//     last_time: Instant,
//     frames_per_second: Duration,
//     game: T,
// }

// impl<T: Game<K, I>, K, I> Application<T, K, I> {
//     pub fn create(game: T) -> Application<T, K, I> {
//         // Start logging
//         initialize_logging();

//         // Return app
//         Application {
//             is_running: false,
//             is_suspended: false,
//             last_time: Instant::now(),
//             frames_per_second: Duration::from_millis(1000 / 2), // 1000 / millis = frames per sec
//             game,
//         }
//     }
//     pub fn start(&mut self) {
//         self.is_running = true;

//         // game configuration
//         self.game.world_config();

//         // game loop
//         while self.is_running {
//             // initial time
//             self.last_time = Instant::now();

//             // process input
//             let random_input = Uid::new_numerical(1);

//             // update
//             self.game.update(random_input as i32);

//             // render

//             // self.last_time += 1.0;
//             let sleep_time = self.frames_per_second - self.last_time.elapsed();
//             thread::sleep(sleep_time);

//             // End the game
//             self.is_running = !self.game.end_game();
//         }
//     }
// }

// pub fn create_app<T: Game>(game: T) -> Application<T> {
//     let app: Application<T> = Application::create(game);
//     app
// }
