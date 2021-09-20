extern crate server;

use server::server::Server;

fn main() {
    Server::new(8888, 4, 4);
}
