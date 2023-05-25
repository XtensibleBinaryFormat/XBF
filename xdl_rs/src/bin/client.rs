use std::io;
use std::net::TcpStream;

use xdl_rs::XdlType;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("localhost:8888")?;

    let my_data = XdlType::U8(69);
    my_data.serialize(&mut stream)?;

    let back_from_server = XdlType::deserialize(&mut stream)?;
    println!("{:?}", back_from_server);
    Ok(())
}
