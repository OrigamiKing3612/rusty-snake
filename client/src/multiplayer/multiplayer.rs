use std::{io::Write, net::TcpStream};

use shared::ClientMessage;

pub fn connect() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    // let message = ClientMessage::Input {
    //     direction: shared::Direction::Up,
    // };
    let message = ClientMessage::Join {
        name: "Player1".to_string(),
    };
    let encoded = bincode::serialize(&message).unwrap();
    stream.write_all(&encoded).unwrap();
}
