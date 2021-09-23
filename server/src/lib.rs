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
extern crate serde;
extern crate serde_json;
extern crate uid;

pub mod game;
pub mod server;
