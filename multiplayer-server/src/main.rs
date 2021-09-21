extern crate blizzard_server;

use blizzard_server::server::Server;

fn main() {
    Server::new(8888, 4, 4);
}
