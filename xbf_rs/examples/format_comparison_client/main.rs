use std::{io::Read, net::TcpStream};

use anyhow::Result;
use byteorder::WriteBytesExt;

#[repr(u8)]
#[derive(Debug)]
enum RequestType {
    Csv,
    MessagePack,
    Cbor,
    Json,
    Xml,
    Xbf,
    Unknown,
}

impl From<u8> for RequestType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Csv,
            1 => Self::MessagePack,
            2 => Self::Cbor,
            3 => Self::Json,
            4 => Self::Xml,
            5 => Self::Xbf,
            _ => Self::Unknown,
        }
    }
}

impl From<RequestType> for u8 {
    fn from(value: RequestType) -> Self {
        value as u8
    }
}

fn main() -> Result<()> {
    for i in 0..RequestType::Unknown.into() {
        let mut connection = TcpStream::connect("ece.stevens.edu:42069")?;
        connection.write_u8(i)?;

        let mut buf = vec![];
        let response = connection.read_to_end(&mut buf)?;

        println!("Request Type: {:?}", RequestType::from(i));
        println!("Received {} bytes back", response);
    }

    Ok(())
}
