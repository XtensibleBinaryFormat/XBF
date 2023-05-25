use std::eprintln;
use std::io;
use std::net::{TcpListener, TcpStream};

use xdl_rs::XdlType;

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr().expect("Stream has peer address");
    eprintln!("New connection from {}", peer_addr);
    let xdl = XdlType::deserialize(&mut stream)?;

    eprintln!("received: {:?}", xdl);

    let to_send = XdlType::String("Thank you for your message!".to_string());
    to_send.serialize(&mut stream)?;
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
