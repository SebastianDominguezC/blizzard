extern crate game;

use game::server::Server;

fn main() {
    Server::new(8888, 4, 2);
}
