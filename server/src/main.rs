use std::io::Read;
use std::net::{TcpListener, TcpStream};

use shared::ClientMessage;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        let size = stream.read(&mut buffer).unwrap();
        if size == 0 {
            break;
        }

        let msg: Result<ClientMessage, _> = bincode::deserialize(&buffer[..size]);

        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to deserialize message: {:?}", e);
                continue;
            }
        };

        match msg {
            ClientMessage::Input { direction } => {
                println!("Received input: {:?}", direction);
            }
            ClientMessage::Join { name } => {
                println!("Player joined: {}", name);
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Server running on 7878");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        std::thread::spawn(|| handle_client(stream));
    }
}
