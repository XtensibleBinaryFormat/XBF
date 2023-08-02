use anyhow::{Context, Result};
use byteorder::WriteBytesExt;
use std::{
    io::{BufReader, BufWriter, Write},
    net::{TcpStream, ToSocketAddrs},
};
use xbf_rs::{XbfMetadata, XbfPrimitive, XbfStruct, XbfType};

fn find_galbatorix(value: &XbfType) -> Option<&XbfStruct> {
    if let XbfType::Vec(v) = value {
        for value in v {
            if let XbfType::Struct(s) = value {
                if let Some(XbfType::Primitive(XbfPrimitive::String(name))) = s.get("name") {
                    if name == "Galbatorix" {
                        return Some(s);
                    }
                }
            }
        }
    }
    None
}

fn get_connection(
    addr: impl ToSocketAddrs,
) -> Result<(BufWriter<TcpStream>, BufReader<TcpStream>)> {
    let stream = TcpStream::connect(addr).context("Unable to connect to server")?;
    let writer = BufWriter::new(stream.try_clone().context("Unable to clone stream")?);
    let reader = BufReader::new(stream);
    Ok((writer, reader))
}

fn get_data_and_metadata(addr: impl ToSocketAddrs) -> Result<(XbfMetadata, XbfType)> {
    let (mut writer, mut reader) = get_connection(addr)?;
    writer.write_u8(0)?;
    writer.flush()?;
    let metadata = XbfMetadata::deserialize_base_metadata(&mut reader)?;
    let value = XbfType::deserialize_base_type(&metadata, &mut reader)?;
    Ok((metadata, value))
}

fn get_data(addr: impl ToSocketAddrs, known_metadata: &XbfMetadata) -> Result<XbfType> {
    let (mut writer, mut reader) = get_connection(addr)?;
    writer.write_u8(1)?;
    writer.flush()?;
    let value = XbfType::deserialize_base_type(known_metadata, &mut reader)?;
    Ok(value)
}

fn unknown_request(addr: impl ToSocketAddrs) -> Result<(XbfMetadata, XbfType)> {
    let (mut writer, mut reader) = get_connection(addr)?;
    writer.write_u8(2)?;
    writer.flush()?;
    let metadata = XbfMetadata::deserialize_base_metadata(&mut reader)?;
    let value = XbfType::deserialize_base_type(&metadata, &mut reader)?;
    Ok((metadata, value))
}

fn main() -> Result<()> {
    let (metadata, value) = get_data_and_metadata("127.0.0.1:6969")?;
    println!("{:?}", find_galbatorix(&value));

    let value = get_data("127.0.0.1:6969", &metadata)?;
    println!("{:?}", find_galbatorix(&value));

    let unnown_request = unknown_request("127.0.0.1:6969")?;
    println!("{:?}", unnown_request);

    Ok(())
}
