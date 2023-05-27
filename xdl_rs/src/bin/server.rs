use std::eprintln;
use std::io;
use std::net::{TcpListener, TcpStream};

use xdl_rs::XdlType;
use xdl_rs::XdlTypeId;

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr().expect("Stream has peer address");
    eprintln!("New connection from {}", peer_addr);
    let xdl = XdlType::deserialize(&mut stream)?;
    dbg!(xdl);
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
