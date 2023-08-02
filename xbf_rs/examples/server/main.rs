mod storage;

use crate::storage::SerializedStorage;
use anyhow::{Context, Result};
use byteorder::ReadBytesExt;
use indexmap::indexmap;
use std::{
    io::{BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};
use xbf_rs::{
    prelude::*, XbfPrimitive, XbfPrimitiveMetadata, XbfStruct, XbfStructMetadata, XbfVec,
    XbfVecMetadata,
};

struct DragonRider {
    name: String,
    age: u16,
}

impl DragonRider {
    fn new(name: String, age: u16) -> Self {
        Self { name, age }
    }

    fn into_xbf_struct(self, metadata: XbfStructMetadata) -> XbfStruct {
        XbfStruct::new_unchecked(
            metadata,
            [
                self.name.into_xbf_primitive().into_base_type(),
                self.age.into_xbf_primitive().into_base_type(),
            ],
        )
    }
}

#[repr(u8)]
enum RequestType {
    GetMetadataAndData = 0,
    GetData,
    Unknown,
}

impl From<u8> for RequestType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::GetMetadataAndData,
            1 => Self::GetData,
            _ => Self::Unknown,
        }
    }
}

fn handle_request(stream: TcpStream, storage: &SerializedStorage) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone().context("Unable to clone stream")?);
    let mut writer = BufWriter::new(stream);

    let request_type = reader.read_u8().context("Unable to read request type")?;
    let request_type = RequestType::from(request_type);

    match request_type {
        RequestType::GetMetadataAndData => storage
            .write_metadata(&mut writer)
            .and_then(|_| storage.write_value(&mut writer)),
        RequestType::GetData => storage.write_value(&mut writer),
        RequestType::Unknown => {
            let unknown_request_message = XbfPrimitive::String(
                "Unknown request type, 0 is data, 1 is data and metadata".to_string(),
            );
            unknown_request_message
                .get_metadata()
                .serialize_primitive_metadata(&mut writer)
                .and_then(|_| unknown_request_message.serialize_primitive_type(&mut writer))
        }
    }
    .context("Unable to write response")?;

    writer.flush().context("Unable to flush response")
}

fn main() -> Result<()> {
    let dragon_riders_storage = {
        let dragon_rider_metadata = XbfStructMetadata::new(
            "DragonRider",
            indexmap! {
                "name" => XbfPrimitiveMetadata::String.into(),
                "age" => XbfPrimitiveMetadata::U16.into(),
            },
        );
        let dragon_riders = [
            DragonRider::new("Eragon".to_string(), 16),
            DragonRider::new("Arya".to_string(), 103),
            DragonRider::new("Galbatorix".to_string(), 133),
        ]
        .map(|dragon_rider| dragon_rider.into_xbf_struct(dragon_rider_metadata.clone()));

        SerializedStorage::new(XbfVec::new_unchecked(
            XbfVecMetadata::new(dragon_rider_metadata),
            dragon_riders,
        ))
    };

    let stream = TcpListener::bind("127.0.0.1:6969").context("Unable to bind tcp listener")?;

    for stream in stream.incoming() {
        let stream_result = stream
            .context("Unable to accept connection")
            .and_then(|stream| handle_request(stream, &dragon_riders_storage));

        if let Err(e) = stream_result {
            eprintln!("{}", e);
        }
    }

    Ok(())
}
