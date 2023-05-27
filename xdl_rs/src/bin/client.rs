use std::io;
use std::net::TcpStream;

use xdl_rs::{XdlType, XdlTypeId, XdlVec};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("localhost:8888")?;

    let raw_data = (0..5).collect::<Vec<u8>>();
    let vec_of_xdl_u8 = raw_data.iter().map(|x| XdlType::U8(*x)).collect();
    let vec_of_xdl_u16 = raw_data.iter().map(|x| XdlType::U16((*x).into())).collect();

    let xdl_vec_of_xdl_u8 = XdlType::Vec(XdlVec::new(vec_of_xdl_u8, XdlTypeId::U8));
    let xdl_vec_of_xdl_u16 = XdlType::Vec(XdlVec::new(vec_of_xdl_u16, XdlTypeId::U16));

    let vec_of_vec = XdlType::Vec(XdlVec::new(
        vec![xdl_vec_of_xdl_u8.clone(), xdl_vec_of_xdl_u16.clone()],
        XdlTypeId::Vec,
    ));

    vec_of_vec.serialize_with_type_id(&mut stream)?;

    Ok(())
}
