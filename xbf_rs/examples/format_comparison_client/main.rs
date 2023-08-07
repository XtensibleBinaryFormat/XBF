use anyhow::Result;
use byteorder::WriteBytesExt;
use std::{io::Read, net::TcpStream, time::Instant};

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
    let addr = format!(
        "{}:42069",
        std::env::args().nth(1).unwrap_or("127.0.0.1".to_string())
    );

    for request_type in 0..RequestType::Unknown.into() {
        for data_format in 0..DataFormat::Unknown.into() {
            let mut durations = vec![];
            let mut bytes_read = None;

            for _ in 0..100 {
                let time_start = Instant::now();

                let mut connection = TcpStream::connect(&addr)?;
                connection.write_u8(request_type)?;
                connection.write_u8(data_format)?;

                let mut buf = vec![];
                let bytes = connection.read_to_end(&mut buf)?;

                let time_elapsed = Instant::now() - time_start;

                if let None = bytes_read {
                    bytes_read = Some(bytes);
                }

                durations.push(time_elapsed);
            }

            let avg_time = durations.iter().sum::<std::time::Duration>() / durations.len() as u32;

            println!(
                "Request Type: {:?}\nData Format: {:?}\nAvg Time: {:?}\nBytes Read: {:?}\n",
                RequestType::from(request_type),
                DataFormat::from(data_format),
                avg_time,
                bytes_read
            );
        }
    }

    Ok(())
}
