use std::eprintln;
use std::io;
use std::net::{TcpListener, TcpStream};

use xdl_rs::LinesCodec;

fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr().expect("Stream has peer address");
    eprintln!("New connection from {}", peer_addr);
    let mut codec = LinesCodec::new(stream).unwrap();

    let message: String = codec.receive_message()?;
    eprintln!("received: {}", message);
    let to_send: String = message.chars().rev().collect();
    codec.send_message(&to_send)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let address = "localhost:8888";
    eprintln!("Starting server on '{}'", address);

    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming().flatten() {
        std::thread::spawn(move || {
            if let Err(e) = handle_connection(stream) {
                eprintln!("Error: {}", e);
            }
        });
    }

    Ok(())
}
