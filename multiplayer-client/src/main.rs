extern crate game;

use game::game::Player;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};
use std::str;
use std::thread;
use std::time::Duration;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to server");
    let port: i32;
    loop {
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
        break;
    }

    stream
        .shutdown(Shutdown::Both)
        .expect("Could not disconnect from original server");

    let tcp = format!("127.0.0.1:{}", port);

    let mut stream = TcpStream::connect(tcp).expect("Could not connect to server");

    loop {
        let mut buffer: Vec<u8> = Vec::new();

        let input = String::from("Game is starting...");

        thread::sleep(Duration::from_millis(1000));

        stream
            .write(input.as_bytes())
            .expect("Failed to write to server");

        let mut reader = BufReader::new(&stream);

        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");

        let json = str::from_utf8(&buffer).unwrap();

        println!("{}", json);

        let player: Vec<Player> = serde_json::from_str(&json).unwrap();

        println!("{:?}", player);
    }
}
