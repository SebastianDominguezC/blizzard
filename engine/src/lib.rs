#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
extern crate uid;
mod core;
mod ecs;
mod game;

pub fn main() {
    core::entry_point::main();
}
