use std::io;
use std::net::TcpStream;

use xdl_rs::{XdlDiscriminant, XdlType, XdlVec};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("localhost:8888")?;

    let raw_data = (0..12).collect::<Vec<u8>>();
    let vec_of_xdl = raw_data.iter().map(|x| XdlType::U8(*x)).collect();

    let xdl_vec = XdlType::Vec(XdlVec::new(vec_of_xdl, XdlDiscriminant::U8));
    xdl_vec.serialize(&mut stream)?;
    dbg!("sent first vector");

    let mut stream = TcpStream::connect("localhost:8888")?;
    let vec_of_vec = XdlType::Vec(XdlVec::new(
        vec![xdl_vec.clone(), xdl_vec.clone(), xdl_vec.clone()],
        XdlDiscriminant::Vec,
    ));
    vec_of_vec.serialize(&mut stream)?;
    dbg!("sent second vector");

    Ok(())
}
