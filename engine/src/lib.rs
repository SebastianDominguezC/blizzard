#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
extern crate uid;
pub mod core;
pub mod ecs;
pub mod game;

use game::Game;

use crate::core::application::create_app;

use std::sync::mpsc;

pub fn start<T: Game<K>, K>(game: T, shared_state: K) {
    let mut app = create_app(game, shared_state);
    let (_, m_receiver) = mpsc::channel();
    app.start(m_receiver);
}
