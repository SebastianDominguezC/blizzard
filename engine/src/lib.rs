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

use crate::core::network_application::create_app;

use game::Game;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

pub fn start_networked<T: Game<K, I>, K, I, M>(
    game: T,
    shared_state: K,
    input: I,
    handle_input: &'static (dyn Fn(Receiver<M>, Arc<Mutex<I>>) -> I + Sync),
) where
    I: Send + Copy,
    M: Send,
{
    let mut app = create_app(game, shared_state, input);
    let (_, m_receiver) = mpsc::channel();

    app.start(m_receiver, handle_input);
}
