#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
extern crate uid;
mod core;
pub mod ecs;
pub mod game;

use game::Game;

pub fn start<T: Game>(game: T) {
    core::entry_point::run(game);
}
