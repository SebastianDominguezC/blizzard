extern crate server;

use server::game::{Player, Position};
use server::server::signal::Signal;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

struct Client {}

impl Client {
    fn start() {
        let mut stream = TcpStream::connect("0.0.0.0:8888").expect("Could not connect to server");
        let port: i32;

        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();

        println!("Enter your username: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        println!("Finding an available lobby...");

        stream
            .write(input.as_bytes())
            .expect("Failed to write to server");

        let mut reader = BufReader::new(&stream);

        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");

        println!("Lobby found");

        println!(
            "{}",
            str::from_utf8(&buffer).expect("Could not write buffer as string")
        );

        port = str::from_utf8(&buffer)
            .expect("Could not write buffer as string")
            .replace("\n", "")
            .parse()
            .expect("Could not parse port");

        if port == 0 {
            println!("No game available, please try again later");
            return;
        }

        stream
            .shutdown(Shutdown::Both)
            .expect("Could not disconnect from original server");

        let tcp = format!("0.0.0.0:{}", port);

        Client::run_game(tcp);
    }

    fn run_game(tcp: String) {
        let mut stream = TcpStream::connect(tcp).expect("Could not connect to server");
        let stream_clone = stream.try_clone().unwrap();

        let position = Arc::new(Mutex::new(Position::new()));
        let position1 = Arc::clone(&position);

        // User Input
        thread::spawn(move || {
            let pos = Arc::clone(&position1);
            loop {
                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read from stdin");
                let mut pos = pos.lock().unwrap();

                let input = input.trim();
                let data: Position;

                if input == "w" {
                    data = Position::up();
                } else if input == "a" {
                    data = Position::left();
                } else if input == "s" {
                    data = Position::down();
                } else if input == "d" {
                    data = Position::right();
                } else {
                    data = Position::new();
                }
                *pos = data;

                let data = Signal::MovePlayer(*pos);
                let json = serde_json::to_string(&data).unwrap() + "\n";

                stream
                    .write(json.as_bytes())
                    .expect("Failed to write to server");
                println!("data written");
            }
        });

        // Stream Reader
        thread::spawn(move || {
            let stream = stream_clone;
            loop {
                let mut buffer: Vec<u8> = Vec::new();
                let mut reader = BufReader::new(&stream);
                reader
                    .read_until(b'\n', &mut buffer)
                    .expect("Could not read into buffer");
                let json = str::from_utf8(&buffer).unwrap();
                let players: Vec<Player> = serde_json::from_str(&json).unwrap();
                println!("{:?}", players);
            }
        });

        // Keep thread alive, so TCP connection on other threads doesn't reset
        loop {}
    }
}

fn main() {
    Client::start();
}
