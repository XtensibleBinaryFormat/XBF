use std::io;
use std::net::TcpStream;

use xdl_rs::LinesCodec;

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("localhost:8888")?;

    let mut codec = LinesCodec::new(stream)?;
    codec.send_message("hello world")?;

    let message = codec.receive_message()?;
    println!("{}", message);

    Ok(())
}
