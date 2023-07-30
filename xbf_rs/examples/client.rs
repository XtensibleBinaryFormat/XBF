use anyhow::{Context, Result};
use byteorder::WriteBytesExt;
use std::{
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
};
use xbf_rs::{XbfMetadata, XbfPrimitive, XbfStruct, XbfType};

fn try_find_galbatorix(value: &XbfType) -> Option<&XbfStruct> {
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

fn main() -> Result<()> {
    let stream = TcpStream::connect("127.0.0.1:6969").context("Unable to connect")?;
    let mut writer = BufWriter::new(stream.try_clone().context("Unable to clone stream")?);
    let mut reader = BufReader::new(stream);

    writer.write_u8(0).context("Unable to write request type")?;
    writer.flush().context("Unable to flush response")?;

    let response_metadata = XbfMetadata::deserialize_base_metadata(&mut reader)?;
    let response_value = XbfType::deserialize_base_type(&response_metadata, &mut reader)?;

    println!("{:?}", try_find_galbatorix(&response_value));

    Ok(())
}
