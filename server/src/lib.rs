//! # Blizzard Server Engine
//!
//! This library along with the Blizzard Game Engine is an all-in-one solution to develop TCP multiplayer games.
//! Please see website to understand how the Blizzard Game and Server engines work!
//! You can also see an example in the GitHub repo, inside the example lib.

#[cfg(test)]
mod tests {
    #[test]
    fn run_game() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_use]
extern crate serde_derive;
extern crate blizzard_engine;
extern crate blizzard_engine_derive;
extern crate blizzard_id;
extern crate serde;
extern crate serde_json;

// Entry point is in server
mod game;
pub mod server;
