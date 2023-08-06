use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Instant,
};

use anyhow::Result;
use byteorder::WriteBytesExt;

#[repr(u8)]
#[derive(Debug)]
enum RequestType {
    Stock,
    Person,
    Unknown,
}

impl From<u8> for RequestType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Stock,
            1 => Self::Person,
            _ => Self::Unknown,
        }
    }
}

impl From<RequestType> for u8 {
    fn from(value: RequestType) -> Self {
        value as u8
    }
}

#[repr(u8)]
#[derive(Debug)]
enum DataFormat {
    Csv,
    MessagePack,
    Cbor,
    Json,
    Xml,
    Xbf,
    Unknown,
}

impl From<u8> for DataFormat {
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

impl From<DataFormat> for u8 {
    fn from(value: DataFormat) -> Self {
        value as u8
    }
}

fn main() -> Result<()> {
    let mut times = vec![vec![vec![]; 6]; 2];

    for _ in 0..500 {
        for request_type in 0..RequestType::Unknown.into() {
            for data_format in 0..DataFormat::Unknown.into() {
                let time_start = Instant::now();

                let mut connection = TcpStream::connect("127.0.0.1:42069")?;
                connection.write_u8(request_type)?;
                connection.write_u8(data_format)?;
                connection.flush()?;

                let mut buf = vec![];
                connection.read_to_end(&mut buf)?;

                let time_elapsed = Instant::now() - time_start;
                times[request_type as usize][data_format as usize].push(time_elapsed);
            }
        }
    }

    for request_type in 0..RequestType::Unknown.into() {
        for data_format in 0..DataFormat::Unknown.into() {
            let entry = &times[request_type as usize][data_format as usize];
            let avg_time = entry.iter().sum::<std::time::Duration>() / entry.len() as u32;
            println!(
                "Request Type: {:?}, Data Format: {:?}, Avg Time: {:?}",
                RequestType::from(request_type),
                DataFormat::from(data_format),
                avg_time
            );
        }
    }

    Ok(())
}
