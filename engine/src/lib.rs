//! # Blizzard Game Engine
//!
//! This library is a custom ECS game engine developed for integration with the Blizzard Server Engine.
//! These two together can be used to develop TCP multiplayer games.
//! However, the idea is that Blizzard will also work for any type of game, not just TCP or online games.
//! Blizzard will be a modular game engine.
//!
//! As of now, it is just an ECS data engine.
//!
//! Some features in the roadmap:
//! * Debugger
//! * Platform abstraction
//! * Window abstraction
//! * Event handler
//! * Renderer API with OpenGL
//! * AI solutions
//! * Many, many more features...
//!

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
extern crate blizzard_id;

pub mod core;
pub mod ecs;
pub mod game;

use game::Game;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use crate::core::network_application::create_app;

/// Start a networked app
/// # Example
/// 1. For example please see the official github repo workspace, inside the example lib
/// 2. TODO: Example is inside src/bin/main.rs
pub fn start_networked<T: Game<K, I>, K, I, M>(
    game: T,
    shared_state: K,
    input: I,
    handle_input: &'static (dyn Fn(Receiver<M>, Arc<Mutex<I>>) -> I + Sync),
) where
    I: Send + Copy,
    M: Send,
{
    let mut app = create_app(game, shared_state, input, 1);
    let (_, m_receiver) = mpsc::channel();

    app.start(m_receiver, handle_input);
}

/// TODO: Start a non-networked app
/// This feature is still missing
pub fn start() {}
